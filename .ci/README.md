# Focused validation

`.ci/ccid.toml` declares the checks and the one job, `verify`; `ccid` runs them and renders the
workflow `.forgejo/workflows/ci.yml` from it. `verify` runs on every push to `main` and on every pull
request of the primary forge: the cached `verify` check (`.ci/verify.sh`) and the live registry
consumers `published-npm` and `published-python` (`.ci/run-cached.sh`). A copy on another forge runs no
CI. The repository commands also work on a suitably provisioned build host.

Registry packages are not made by `verify`. The release run packs the crate, the npm tarball and the
PyPI files once, from the commit, with the `pack` check of `[release]`; see
[releasing](../docs/releasing.md).

Select checks from changed inputs and missing evidence. A changed README does not
require another Rust build. Table/API changes need affected language vectors;
package metadata, exports and dependency changes need installed-package checks.
Use a broader selection when the impact is unknown. Existing results are reusable
only for matching source, dependency closure, tools, configuration and environment.

| Selector | Coverage |
| --- | --- |
| `metadata` | Version alignment and byte-for-byte generated assets |
| `fmt` | Rust formatting only |
| `rust` | Locked Rust tests including doctests, formatting, Clippy with denied warnings |
| `javascript` | Strict TypeScript and shared source vectors |
| `rust-package` | Build and verify the registry-resolved Cargo package |
| `js-package` | Pack once; installed npm/Node/Bun imports and declarations, browser module, JSR dry-run |
| `js-pnpm`, `js-yarn`, `js-bun` | Selected additional manager consuming the same verified npm tarball |
| `python-package` | Wheel/sdist, installed-wheel vectors, metadata and JSON CLI consumers |
| `typst-package` | Deterministic archive and actual installed Typst import |
| `rust-dependencies` | Pre-publication source tests using checksum-verified sibling crates in private scratch |
| `published-npm`, `published-python` | Consumers of the exact version on the selected live registry; a version that is not published yet is skipped |

For example, plan and run a selection:

```sh
ccid check --repo . --check metadata,rust --plan
ccid check --repo . --check metadata,rust
```

To check every source and distribution selector:

```sh
ccid check --repo . \
  --check metadata,rust,javascript,rust-package,js-package,python-package,typst-package
```

With `ARTIFACT_ROOT` set to an absolute worker path, each package selector
exports immutable files under `<root>/<package>/<commit>/`, plus its own JSON
receipt, `SOURCE_COMMIT`, and an aggregate `SHA256SUMS`. Additional manager checks
require `js-package` evidence for that exact commit. They never rebuild the tarball.
Use them when manager/export/packaging behavior changed; do not repeat all managers
for every source edit. These files are verification evidence: the files that are
published come from the release run's `pack` check.

For unpublished sibling dependencies, supply `DEPENDENCY_MANIFEST` and
`DEPENDENCY_MANIFEST_SHA256`. The manifest is an exact JSON document:

```json
{
  "schema": 1,
  "artifacts": [
    {
      "kind": "wheel",
      "name": "cgreet",
      "path": "/absolute/path/to/verified/cgreet.whl",
      "sha256": "the actual 64-character lowercase SHA-256"
    }
  ]
}
```

Kinds are `wheel`, `crate`, or `npm`; each consuming check selects its own kind.
Python installs the specified sibling wheels alongside its own wheel. The
`rust-dependencies` selector copies source and verified sibling crates to owned
scratch, uses temporary Cargo patches and a scratch-only lock, and runs the tests.
This is integration evidence, not registry package verification. `rust-package`
always uses normal registry resolution and requires published dependencies.
Neither the source tree nor the released manifest acquires local path patches.

The shared runner bounds concurrency to two build/test threads by default and
selected checks to 900 seconds; `CI_JOBS`, `CI_TEST_THREADS`, and `CI_TIMEOUT` can
supply a concrete different budget. It inherits dedicated package caches and
namespaces Cargo targets by canonical repository identity. It verifies source and
tool digests, preserves unchanged source freshness, supervises timeout/cancellation,
and never clears caches or installs system tools. Commands that generate source or
install dependencies run in owned scratch copies, preserving the verified input
tree and its reusable Rust timestamps.

Inspect existing runs before dispatch. After a failed Python check, select Python
again after correcting its inputs; a passed unrelated Rust check remains evidence.
Keep exact check receipts and durations when assessing whether wider validation is
needed. Linux results do not establish native Windows/macOS behavior or another
Rust/Node/Python version. Unavailable checks remain explicitly unverified.
