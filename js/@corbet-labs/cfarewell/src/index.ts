/**
 * Deterministic locale-correct valedictions for formal correspondence.
 *
 * Pure TypeScript port of the cfarewell Rust crate: zero dependencies, zero
 * Node APIs, synchronous, no I/O. Behavior is defined by `tables/*.json`
 * at the repository root; `tests/vectors/*.json` is the shared conformance
 * suite.
 */
import { CLOSING_TABLE } from './generated/tables.ts';

export interface ClosingTable {
    locales: Record<string, string>;
    fallback: string;
}

const table = CLOSING_TABLE as ClosingTable;

function baseLanguage(locale: string): string {
    return locale.split('-')[0] ?? locale;
}

function resolveKey(locale: string): string {
    const lower = locale.toLowerCase();
    if (Object.hasOwn(table.locales, lower)) return lower;
    const base = baseLanguage(lower);
    return Object.hasOwn(table.locales, base) ? base : table.fallback;
}

/**
 * Valediction for a BCP 47 locale. Unknown locales fall back through the
 * base language to English; an explicit override always wins (used for
 * per-workspace closing choices).
 */
export function closing(locale: string, overrideClosing?: string): string {
    if (overrideClosing !== undefined) return overrideClosing;
    return table.locales[resolveKey(locale)] ?? '';
}

/** BCP 47 locale codes with a valediction entry, sorted. */
export function availableLocales(): string[] {
    return Object.keys(table.locales).sort();
}
