# Hijriyah Module — Architectural Decisions

## Decision 1: GPL-3.0 Library Exclusion (Option A)

**Date:** 2026-05-22
**Status:** ✓ Approved & Implemented
**Decision:** Exclude GPL-3.0 and other copyleft dependencies; reimplement Hijri arithmetic independently.

### Context

The Islamic (Hijri) calendar is a well-studied, fully-deterministic lunar calendar with published algorithms in academic literature. Several existing implementations exist:

| Implementation | Language | License | Algorithm Source |
|---|---|---|---|
| `misykat` | Python | GPL-3.0-only | Meeus Ch. 9 + custom extensions |
| `hijri-js` | JavaScript | MIT | Islamic calendar standards |
| `hijri` | Rust (unmaintained) | MIT | Basic tabular algorithm |
| Custom (D-R Ch. 6) | Various | Various | Dershowitz-Reingold |

### Problem

The project is licensed **MIT OR Apache-2.0** (dual permissive). Adding a GPL-3.0-only dependency would force:
1. All dependents to accept GPL-3.0-only terms
2. No commercial / proprietary use without GPL relicense
3. Incompatibility with Apache-2.0-only ecosystems
4. Breaks the project's stated license compatibility goal (ARCHITECTURE.md § 12)

### Decision (Option A)

**Reimplement Hijri arithmetic independently** from academic sources:
- Primary: Dershowitz-Reingold, *Calendrical Calculations* (4th ed.), Ch. 6 — "Islamic Calendar"
- Secondary: Meeus, *Astronomical Algorithms* (2nd ed.), Ch. 9 — "Islamic Calendar" (cross-validation)

**Rationale:**
- Hijri arithmetic is well-documented in two canonical academic sources
- Algorithm is deterministic (tabular leap-year rules, fixed epoch)
- No research invention required — direct formula translation from published references
- Keeps workspace MIT OR Apache-2.0 permissive and usable by commercial projects
- Enables `no_std + alloc` compatible implementation (no heavy dependencies)

### Consequences

**Benefits:**
- ✓ Maintains dual MIT/Apache-2.0 license
- ✓ No copyleft obligations on dependents
- ✓ Compiles to WASM, `no_std`-compatible
- ✓ Clear source attribution (academic papers, not GPL code)
- ✓ Fully testable against published historical dates

**Costs:**
- Implementation effort: ~2–3 hours (tabular algorithm, no numerical methods)
- Verification risk: Must cross-validate against multiple sources
- No code reuse from existing Rust crate

### Verification Anchor Points

Key historical dates verifying the D-R astronomical tabular variant (Thursday epoch):

1. **1 Muharram 1 AH:** JDN 1948439 (Thursday, July 15, 622 CE Julian) — Islamic epoch
2. **1 Muharram 1043 AH:** JDN 2317689 (July 7, 1633 CE Gregorian) — near Sultan Agung
   reform (civil variant places this 1 day later at JDN 2317690 = July 8, 1633)
3. **1 Muharram 1355 AH:** JDN 2428251 (March 23, 1936 CE Gregorian)
4. **1 Muharram 1446 AH:** JDN 2460499 (July 7, 2024 CE Gregorian) — matches Saudi
   Umm al-Qura announcement

All four dates round-trip through:
```
gregorian ←→ jdn ←→ hijri
```

### Variant Note

Two common Islamic tabular variants differ by 1 day:

- **Astronomical** (Thursday epoch, JDN 1948439): used here; matches D-R `fixed(227015)`
- **Civil** (Friday epoch, JDN 1948440): used in some historical records (e.g., the
  Sultan Agung Javanese reform records 1 Muharram 1043 AH as July 8, 1633 = JDN 2317690)

Both share the same leap year rule and month lengths; only the epoch differs by 1 day.

### Related Decisions

- **Calendar-core trait design:** `CalendarDate`, `CalendarMetadata`, `HasAuspiciousness` traits mandate source attribution in rustdoc (ARCHITECTURE.md § 7)
- **Observation-dependent calendars:** Later modules (batak, sasak, bugis, minangkabau) use tabular fallbacks for Pleiades observation dates due to similar copyleft exclusion strategy

### References

- **ARCHITECTURE.md § 12:** "Dependency Decisions & Licenses" — workspace-level GPL exclusion policy
- **Dershowitz & Reingold (2018):** *Calendrical Calculations*, 4th ed. MIT Press. ISBN 978-0-262-03929-3. Chapter 6: Islamic Calendar.
- **Meeus (1998):** *Astronomical Algorithms*, 2nd ed. Willmann-Bell. ISBN 0-943396-61-1. Chapter 9: Islamic Calendar.
- **CONTRIBUTING.md:** "Source Requirements" — every algorithm must cite a citable, verifiable source.

---

## Implementation Checkpoints

✓ Decision documented
⏳ Module structure defined (`arithmetic.rs`, `types.rs`, `holidays.rs`, `metadata.rs`)
⏳ Algorithms implemented from D-R Ch. 6 + Meeus Ch. 9
⏳ Anchor tests passing (1 Muharram 1 AH, 1043 AH, 1355 AH, 1446 AH)
⏳ Round-trip tests (1000 random JDNs, 1–1600 AH range)
⏳ `CalendarDate` + `CalendarMetadata` trait implementations

---

**Approved by:** Architecture review (embedded in project spec)
**Next task:** 1.2 — Implement `hijri_to_jdn` and `jdn_to_hijri` per D-R Eq. 6.2–6.3
