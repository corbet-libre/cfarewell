# Changelog

All notable changes to `cfarewell` are documented here. The project follows
Semantic Versioning.

## 0.3.0 - Unreleased

- License this new release line under LGPL-3.0-only WITH LGPL-3.0-linking-exception across Cargo, npm, JSR,
  Python and Typst, with the complete LGPL and incorporated GPL notices.
- Keep runtime behavior, correspondence tables and dependency versions unchanged.

## 0.2.1 - 2026-09-11

- Align package metadata and packaged license texts across Cargo, npm, JSR, Python, and Typst.
- Keep runtime behavior and dependency versions unchanged; previously published releases retain their original licenses and artifacts.

## 0.2.0 - 2026-09-09

- Reuse packed distributions and run selected checks through Crow.
- **Breaking:** returned locale IDs are lowercase; mixed-case inputs remain accepted. Add Liechtenstein German (`de-li`).

- Ship compiled ESM and CommonJS, declaration files, and a standalone browser module.
- Add a Python distribution with shared-vector conformance and a JSON CLI.
- Add JSR packaging, installed-artifact tests, and complete registry license files.
- Clarify scope, examples, installation options, and family links on the product page.
- Resolve locale codes case-insensitively, preserving regional defaults.

## 0.1.1 - 2026-09-06

- Satisfy the pinned 1.94.0 Clippy gate (`unwrap_or_else` over
  `map_or_else` with an identity closure). No behavior change.

## 0.1.0 - 2026-09-06

- Initial release: locale-correct valedictions for 30 locales (BCP 47,
  exact → base-language → English fallback, explicit override), as a Rust
  crate, a pure-TypeScript package, and a Typst module sharing one table
  and one vector suite.
