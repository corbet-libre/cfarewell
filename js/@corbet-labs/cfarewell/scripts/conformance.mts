/**
 * TypeScript side of the cross-language conformance gate: runs every
 * `tests/vectors/*.json` vector through `src/index.ts` and compares with
 * `expected` exactly. Exits non-zero with the first mismatch.
 *
 * Run from the package root:
 *   bun ./scripts/conformance.mts
 */
import { readdirSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';
import { closing, availableLocales } from '../src/index.js';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');

interface Vector {
    name: string;
    fn: string;
    locale: string;
    override?: string;
    expected: unknown;
}

function runVector(file: string, vector: Vector): void {
    const what = `${file} :: ${vector.name}`;
    let actual: unknown;
    switch (vector.fn) {
        case 'available_locales':
            actual = availableLocales();
            break;
        case 'closing':
            actual = closing(vector.locale, vector.override);
            break;
        default:
            throw new Error(`${what}: unknown fn ${vector.fn}`);
    }
    const got = JSON.stringify(actual) ?? 'undefined';
    const want = JSON.stringify(vector.expected) ?? 'undefined';
    if (got !== want) throw new Error(`${what}: ${got} vs ${want}`);
}

const dir = join(ROOT, 'tests/vectors');
const files = readdirSync(dir)
    .filter((file) => file.endsWith('.json'))
    .sort();
if (files.length === 0) throw new Error('no vector files in tests/vectors');
let count = 0;
for (const file of files) {
    const vectors = JSON.parse(readFileSync(join(dir, file), 'utf8')) as Vector[];
    for (const vector of vectors) {
        runVector(file, vector);
        count += 1;
    }
}
console.log(`TS conformance green: ${count} vectors across ${files.length} files`);
