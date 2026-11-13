# Releasing cfarewell

Keep Cargo.toml, Cargo.lock, package.json, jsr.json, pyproject.toml, and
typst.toml versions consistent. Published contents are immutable: use a new
version for corrections.

1. Update versions and CHANGELOG.md (add a `## X.Y.Z - date` section; the text
   of CHANGELOG.md, at most 64 KiB, is the release notes), then regenerate
   canonical tables with `bash js/@corbet-labs/cfarewell/scripts/sync-assets.sh`.
   Commit to `main`.
2. Dispatch the `release` workflow on `main` of the primary forge. ccid reads
   the version from `Cargo.toml`; the versions in `Cargo.toml`, `package.json`
   and `pyproject.toml` must agree, or the pack refuses the release before
   anything is public. The run:
   - checks that the source is the tip of `main`, that `vX.Y.Z` does not exist
     yet and that the commit's verify is not red;
   - packs the crate, the npm tarball and the PyPI files once, from the
     commit, with the `pack` check, and collects them in one draft;
   - promotes the draft to the release `vX.Y.Z` with a `SHA256SUMS` file and
     `CHANGELOG.md` as its notes.
3. The datacenter-side publisher downloads that release, checks the files
   against `SHA256SUMS` and uploads exactly them to crates.io, npm and PyPI.
   Copies on other forges get the same files; no CI runs on a copy.
4. Run the published-consumer checks (`published-npm`, `published-python`) and
   state the status per registry. A successful upload response or version
   collision is insufficient proof.

Publish sibling dependencies before cletter and resolve its committed locks
against those registry versions. Local path overrides are integration evidence.

## What is packed

The `pack` check in `.ci/ccid.toml` names the three package directories (`.`,
`js/@corbet-labs/cfarewell` and `py`). The release run packs from a clean
checkout of the commit with the committed locks, so nothing is resolved at
release time except what a lock names:

- `cargo package --locked`: `cfarewell-X.Y.Z.crate`;
- `npm pack` after a frozen install: `corbet-labs-cfarewell-X.Y.Z.tgz`;
- `uv build`: the source distribution and the wheel.

The Python build backend is pinned by hashes in `py/build-constraints.txt`.
Refresh it together with the other locks:

```sh
cd py
python3 -c 'import tomllib;print(*tomllib.load(open("pyproject.toml","rb"))["build-system"]["requires"],sep="\n")' \
  | uv pip compile --universal --generate-hashes --no-config --python-version 3.10 - -o build-constraints.txt
```

The same commit packs to the same bytes. A pack that fails stops the release
before anything is public; fix `main` and dispatch again. A rerun for the same
commit resumes its draft.

JSR is not published for now. `jsr.json` stays, and the `js-package` check still
dry-runs it. The Typst archive is checked by `typst-package` but is not a file of
the release; Typst Universe requires a separate eligible submission, see
[installation](installation.md).

The npm registry serves npm, pnpm, Yarn, Bun, Deno's npm imports, and browser
CDNs. A successful Linux check does not prove native Windows/macOS behavior.
Minimum-runtime checks are separate from current-runtime checks. Record any
unavailable runtime or registry explicitly instead of marking it verified.
