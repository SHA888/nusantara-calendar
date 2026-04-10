# Contributing to `nusantara-calendar`

Thank you for helping document and preserve the calendrical knowledge of the Indonesian archipelago.
Every contribution must be sourced, reproducible, and license-clean.

## Table of Contents

1. [Guiding Principles](#guiding-principles)
2. [Source Requirements](#source-requirements)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Requirements](#testing-requirements)
6. [License Expectations](#license-expectations)

---

## Guiding Principles

- **Algorithmic honesty.** If an algorithm is undocumented, add a `stub!()` with a citation instead of guessing.
- **Julian Day Number as pivot.** All calendar conversions must go through JDN for interoperability.
- **`no_std + alloc` first.** Core arithmetic crates must compile without `std` unless explicitly documented.
- **Attribution is mandatory.** Every public API surface needs an academic or primary-source citation.

## Source Requirements

- Cite primary literature where available (academic papers, ethnographic texts, government almanacs).
- Include page numbers or sections in doc comments when practical.
- Add new references to the relevant `SOURCES.md` file within each crate and, if broadly applicable, to the workspace-wide bibliography (planned).
- Oral-tradition knowledge must reference published fieldwork or official community statements; otherwise keep it as a stub describing what is known.

## Development Workflow

1. **Discuss first** via GitHub issue or discussion thread for anything non-trivial.
2. **Fork or branch** from `main`; keep branches focused on a single crate or feature.
3. **Keep commits small and well-described.** Use prefixes like `feat:`, `fix:`, `docs:`, `refactor:`.
4. **Follow the /scp or /tpscp workflow** when interacting via Windsurf (stage → commit → push, or test → pre-commit → stage → commit → push).
5. **Run the required tests** (see below) before opening a pull request.
6. **Reference sources in the PR description** and highlight any licensing considerations.
7. **Expect review.** We check citations, `no_std` compatibility, and WASM builds.

## Coding Standards

- Rust 2024 edition, MSRV 1.94.
- Use `calendar-core` traits (`CalendarDate`, `CalendarMetadata`) for all date structs.
- Keep modules focused: arithmetic, types, holidays, metadata, tests.
- Avoid introducing new dependencies without documenting their licenses in `ARCHITECTURE.md`.
- Use `alloc::string::String` for error messages in `no_std` contexts.
- Prefer `const fn` for pure arithmetic helpers where possible.

## Testing Requirements

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets` (when lint config is added)
- `cargo test --workspace`
- `cargo test --workspace --no-default-features`
- `cargo build --target wasm32-unknown-unknown -p <crate> --no-default-features` for every `no_std` crate
- Any crate-specific anchor or property tests described in `SPEC.md`

## License Expectations

- All contributions are accepted under the workspace license: `MIT OR Apache-2.0`.
- Do **not** introduce GPL or other copyleft dependencies without prior approval and documentation.
- When adding third-party data or text, ensure it is compatible with permissive licensing and cite the source.

---

By contributing, you affirm that your work complies with these requirements and that you have the rights to submit it under the project’s dual license.
