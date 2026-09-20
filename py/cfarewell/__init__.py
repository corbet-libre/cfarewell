"""Locale-correct valedictions with explicit override support."""

from ._tables import TABLES

_TABLE = TABLES["closing"]
_KEYS = {key.lower(): key for key in _TABLE["locales"]}
__all__ = ["closing", "available_locales"]


def closing(locale: str, override_closing: str | None = None) -> str:
    """Resolve exact locale, base language, then English; an override wins."""
    if override_closing is not None:
        return override_closing
    lowered = locale.lower()
    key = _KEYS.get(lowered, _KEYS.get(lowered.split("-", 1)[0], _TABLE["fallback"]))
    return _TABLE["locales"][key]


def available_locales() -> list[str]:
    """Sorted canonical BCP 47 codes in the valediction table."""
    return sorted(_TABLE["locales"])
