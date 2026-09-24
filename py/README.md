# cfarewell

> **Superseded by [`cnice`](https://github.com/corbet-foss/cnice).**
> New development continues as `cnice.farewell` (same API, same vectors);
> this crate stays frozen at 0.3.1 and published grants are unchanged.

**The right closing line for a formal letter.**

[![crates.io](https://img.shields.io/crates/v/cfarewell.svg)](https://crates.io/crates/cfarewell) [![npm](https://img.shields.io/npm/v/@corbet-labs/cfarewell.svg)](https://www.npmjs.com/package/@corbet-labs/cfarewell) [![PyPI](https://img.shields.io/pypi/v/cfarewell.svg)](https://pypi.org/project/cfarewell/) [![Rust API](https://docs.rs/cfarewell/badge.svg)](https://docs.rs/cfarewell)

Choose a valediction from 40 locale entries, including Swiss German, British English, French, and Japanese. Explicit wording always takes precedence over table defaults. No model or network calls.

```js
import { closing } from '@corbet-labs/cfarewell';

closing('de-ch');
// Freundliche Grüsse
```

## Install

| Environment | Command |
| --- | --- |
| Rust / Cargo | `cargo add cfarewell` |
| Python / pip | `python -m pip install cfarewell` |
| Python / uv | `uv add cfarewell` |
| Node.js / npm | `npm install @corbet-labs/cfarewell` |
| pnpm | `pnpm add @corbet-labs/cfarewell` |
| Yarn | `yarn add @corbet-labs/cfarewell` |
| Bun | `bun add @corbet-labs/cfarewell` |
| Deno | `deno add npm:@corbet-labs/cfarewell` |

The 0.2.1 JavaScript distribution includes compiled ESM, CommonJS,
TypeScript declarations, and a standalone browser module. Node.js 20+ is
supported; no TypeScript loader is required.

```js
// CommonJS
const { closing } = require('@corbet-labs/cfarewell');
```

```html
<script type="module">
  import { closing } from 'https://cdn.jsdelivr.net/npm/@corbet-labs/cfarewell@0.2.1/dist/browser.js';
  console.log(closing('de-ch'));
</script>
```

Python 3.10+ packages are available on [PyPI](https://pypi.org/project/cfarewell/).
See the [installation guide](https://github.com/corbet-foss/cfarewell/blob/main/docs/installation.md)
for CLI commands and other distribution options.
JSR publication and Typst availability are listed there explicitly.

## Rust

```rust
use cfarewell::closing;

assert_eq!(closing("de-ch", None), "Freundliche Grüsse");
assert_eq!(closing("en-gb", None), "Yours sincerely,");
```

## Python

```python
from cfarewell import closing

assert closing("de-ch") == 'Freundliche Grüsse'
```

## API

| Function | Purpose |
| --- | --- |
| `closing(locale, override?)` | Resolve a valediction; even an empty override wins |
| `availableLocales()` / `available_locales()` | List the 40 canonical locale codes |

Locale matching is case-insensitive. Resolution tries the exact locale, then its base language, then English: `fr-be` → `fr`, `xx` → `en`. An override is returned unchanged, including an empty string. The locale table owns punctuation, so callers should not append another comma.

## Correspondence family

| Library | Responsibility |
| --- | --- |
| [cletter](https://github.com/corbet-foss/cletter) | Compose the correspondence helpers |
| [cgreet](https://github.com/corbet-foss/cgreet) | German salutations and titles |
| [cfarewell](https://github.com/corbet-foss/cfarewell) | Locale-specific closings |
| [cdate](https://github.com/corbet-foss/cdate) | Calendar-date formatting |
| [cink](https://github.com/corbet-foss/cink) | Handwritten signature images |


## Development

Behavior is defined by [the locale tables](https://github.com/corbet-foss/cfarewell/tree/main/tables)
and [shared conformance vectors](https://github.com/corbet-foss/cfarewell/tree/main/tests/vectors).
Rust, JavaScript, and Python run the same vectors. Selected CI checks exercise
installed JavaScript tarballs, Python wheels and command-line entrypoints, and
Typst packages. Release validation records the actual runtime and platform;
Linux results do not establish native Windows or macOS coverage.
All five Rust crates forbid unsafe code in their own source.

See [the release guide](https://github.com/corbet-foss/cfarewell/blob/main/docs/releasing.md)
for generation, verification, and publication commands.

## License

Copyright 2026 Julian Y. Richard Corbet. The 0.3.1 release line is licensed
under [LGPL-3.0-only](https://github.com/corbet-foss/cfarewell/blob/main/LICENSES/LGPL-3.0-only.txt)
[WITH LGPL-3.0-linking-exception](https://github.com/corbet-foss/cfarewell/blob/main/LICENSES/LGPL-3.0-linking-exception.txt),
with the incorporated [GPL version 3](https://github.com/corbet-foss/cfarewell/blob/main/LICENSES/GPL-3.0-only.txt).
Combined works may link statically or dynamically without relinking duties;
library modifications stay LGPL. Applications can use a different license
subject to the LGPL's conditions.
Version 0.2.1 retains Apache-2.0. The installation examples above refer to those available
releases; 0.3.1 is published to registries.

See the [licensing notes](https://github.com/corbet-foss/cfarewell/blob/main/LICENSE.md) for distribution conditions and retained notices.
Contributions use the [Contributor License Agreement](https://github.com/corbet-foss/cfarewell/blob/main/CLA.md).
