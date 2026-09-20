// Cross-port conformance: runs every `tests/vectors/*.json` vector through
// `typst/farewell.typ` and compares with `expected` exactly. A failing
// vector aborts compilation with its `file :: name`.
//
// Compile from the repository root:
//   typst compile --root . tests/conformance.typ /tmp/cfarewell-conformance.pdf
#import "../typst/farewell.typ" as api

#let run-vector(file, vector) = {
  let what = file + " :: " + vector.at("name", default: "<unnamed>")
  let actual = if vector.fn == "available_locales" {
    api.available-locales()
  } else if vector.fn == "closing" {
    api.closing(vector.locale, override: vector.at("override", default: none))
  } else {
    panic("unknown fn " + repr(vector.fn) + " in " + what)
  }
  assert.eq(actual, vector.at("expected", default: none), message: what)
}

#let run-file(file) = {
  let vectors = json("vectors/" + file)
  for vector in vectors {
    run-vector(file, vector)
  }
  vectors.len()
}

#let total = run-file("closing.json") + run-file("locale_policy.json") + run-file("lowercase.json")

Typst conformance green: #total vectors across 3 files.
