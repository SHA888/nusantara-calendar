# Source Verification — `nusantara-calendar`

This document tracks the primary sources for each calendar module and their verification status.

## Legend

| Symbol | Tier | Meaning |
|--------|------|---------|
| ✅ | Primary + Verifiable | Peer-reviewed publication with accessible algorithm; can be independently verified |
| ⚠️ | Primary + Print-Only | Authoritative but requires physical book access or manual transcription |
| ℹ️ | Cross-Check Only | Useful for validation but not primary algorithm source |
| ❌ | Unverified | Source mentioned but not yet validated against implementation |

---

## `jawa` Module — Javanese Calendar

| Source | Year | Verification Tier | Notes |
|--------|------|-------------------|-------|
| Karjanto & Beauducel (arXiv:2012.10064) | 2020 | ✅ | Primary + digitally verifiable. Wetonan congruence formulas. Directly citable, arXiv provides permanent URL. |
| Dershowitz & Reingold, *Calendrical Calculations* (4th ed.) | 2018 | ✅ | Primary + widely cited. Pawukon 210-day algorithm (Ch. 10). Standard reference for calendar computation. |
| H. Danudji, *Penanggalan Jawa 120 Tahun Kurup Asapon* | 2006 | ⚠️ | Primary, print-only (Dahara Prize). ISBN 979-501-454-4. Complete Kurup tables including supra-windu groups (Adi, Kuntara, Sengara, Sancaya). Requires triangulation with secondary sources due to no digital access. |
| Wikipedia "Javanese calendar" | 2026 | ℹ️ | Cross-check only. Good for anchor validation (e.g., 1936-03-24 = Selasa Pon) but lacks algorithmic detail for implementation. |
| `beaudu/weton` GitHub repository | 2020 | ℹ️ | Cross-check only. MATLAB/Perl/C reference implementation. BSD-licensed. Used to validate formulas against working code, but not a primary academic source. |

### Algorithm-to-Source Mapping

| Algorithm | Primary Source | Verification Method |
|-----------|---------------|---------------------|
| Pasaran (5-day): `jdn.rem_euclid(5)` | Karjanto & Beauducel (2020), Eq. derivation | Matches `beaudu/weton` output at epoch |
| Saptawara (7-day): `jdn.rem_euclid(7)` | Karjanto & Beauducel (2020) | Matches `beaudu/weton`; epoch = Jemuwah (4) |
| Wuku (30-week): `((jdn/7)+12).rem_euclid(30)` | Dershowitz-Reingold Ch. 10 | Epoch = Sinta (0) verified |
| Windu year: `(aj-1)%8` → 8 names | Karjanto & Beauducel (2020) | Leap years: Jimawal, Dal, Jimakir |
| Kurup boundaries | Danudji (2006) + Wikipedia triangulation | 1936-03-24 → 2052-08-25 (120 years) |
| Lunar months (Wulan) | Tabular from `beaudu/weton` windu tables | 354/355 day years validated |
| Pranata Masa | Solar approximation (365-day cycle) | June solstice anchor (~June 21) |
| Supra-windu groups (Adi, Kuntara, Sengara, Sancaya) | **STUB** — Danudji (2006) required | Not yet implemented; enum defined |

### Confidence Assessment

- **High confidence**: Wetonan, Pawukon, Windu year (mathematically verified, multiple sources agree)
- **Medium confidence**: Kurup boundaries (triangulated, but windu-year-to-AJ mapping needs Danudji table)
- **Pending verification**: Supra-windu group algorithm (requires full Kurup table transcription)

---

## Other Modules

*(To be populated as implementation proceeds)*

- `balinese` — see `balinese-calendar` crate documentation
- `hijriyah` — (in progress)
- `chinese_nusantara` — (in progress)
- `batak`, `sunda`, `tengger`, etc. — (pending)

---

## Maintenance

When adding a new source:
1. Assign verification tier based on accessibility and authority
2. Document ISBN/DOI/arXiv ID for permanent access
3. Note if triangulation with other sources was required
4. Update algorithm-to-source mapping when implementing new features

When marking `stub!()` → implementation:
1. Verify algorithm against primary source before removing stub
2. Add test case with known anchor from source
3. Update this document with verification method
