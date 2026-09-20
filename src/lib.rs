//! Deterministic locale-correct valedictions for formal correspondence.
//!
//! The valediction per locale lives as data in `tables/closing.json` (see
//! `tables/README.md` for the resolution rule); `tests/vectors/*.json` is
//! the executable contract every language port runs. The Typst module in
//! `typst/` is generated from the same table.
//!
//! Same input always yields the same output: no models, no I/O.

use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(serde::Deserialize)]
struct ClosingTable {
    locales: HashMap<String, String>,
    fallback: String,
}

static TABLE: LazyLock<ClosingTable> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../tables/closing.json"))
        .expect("tables/closing.json is valid")
});

fn base_language(locale: &str) -> &str {
    locale.split('-').next().unwrap_or(locale)
}

/// Resolve a lowercase table key: case-insensitive exact code, base language,
/// then English fallback. All stored and returned locale IDs are lowercase.
fn resolve_key(locale: &str) -> &'static str {
    let lower = locale.to_ascii_lowercase();
    TABLE
        .locales
        .get_key_value(lower.as_str())
        .or_else(|| TABLE.locales.get_key_value(base_language(&lower)))
        .map_or(TABLE.fallback.as_str(), |(key, _)| key.as_str())
}

/// Valediction for a BCP 47 locale. Unknown locales fall back through the
/// base language to English; an explicit override always wins (used for
/// per-workspace closing choices).
#[must_use]
pub fn closing<'a>(locale: &str, override_closing: Option<&'a str>) -> &'a str {
    override_closing.unwrap_or_else(|| {
        TABLE
            .locales
            .get(resolve_key(locale))
            .map_or("", String::as_str)
    })
}

/// BCP 47 locale codes with a valediction entry, sorted.
#[must_use]
pub fn available_locales() -> Vec<&'static str> {
    let mut codes: Vec<&'static str> = TABLE.locales.keys().map(String::as_str).collect();
    codes.sort_unstable();
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_resolution_is_case_insensitive() {
        assert_eq!(closing("EN", None), "Yours sincerely,");
        assert_eq!(closing("DE", None), "Mit freundlichen Grüßen");
        assert_eq!(closing("en-ca", None), "Yours sincerely,");
        assert_eq!(closing(" ", None), "Yours sincerely,");
        assert_eq!(closing("de", Some("")), "");
    }

    #[test]
    fn tables_schema() {
        assert!(!TABLE.locales.is_empty(), "table must not be empty");
        assert!(
            TABLE.locales.contains_key(TABLE.fallback.as_str()),
            "fallback must be a known locale"
        );
        for key in TABLE.locales.keys() {
            let base = base_language(key);
            assert!(!base.is_empty(), "locale key must not be empty");
            assert_eq!(
                key,
                &key.to_ascii_lowercase(),
                "locale ID must be lowercase: {key:?}"
            );
        }
        for value in TABLE.locales.values() {
            assert!(!value.is_empty(), "valediction must not be empty");
        }
    }

    fn run_vector(file: &std::path::Path, vector: &serde_json::Value) {
        let name = vector["name"].as_str().unwrap_or("<unnamed>");
        let context = format!("{} :: {name}", file.display());
        let actual: serde_json::Value = match vector["fn"].as_str().unwrap_or("") {
            "available_locales" => serde_json::to_value(available_locales()).unwrap(),
            "closing" => {
                let locale = vector["locale"]
                    .as_str()
                    .expect("closing vector needs locale");
                let override_closing = vector.get("override").and_then(serde_json::Value::as_str);
                serde_json::Value::String(closing(locale, override_closing).to_owned())
            }
            other => panic!("{context}: unknown fn {other:?}"),
        };
        let expected = vector
            .get("expected")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, expected, "{context}");
    }

    #[test]
    fn conformance_vectors() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vectors");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
            .expect("tests/vectors exists")
            .map(|entry| entry.expect("readable entry").path())
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no vector files in tests/vectors");
        let mut count = 0;
        for file in &files {
            let raw = std::fs::read_to_string(file).expect("vector file is readable");
            let vectors: Vec<serde_json::Value> =
                serde_json::from_str(&raw).expect("vector file is valid JSON");
            for vector in &vectors {
                run_vector(file, vector);
                count += 1;
            }
        }
        assert!(count > 0, "no vectors ran");
    }
}
