# Repository Guidelines

## Project Structure & Module Organization
The formatter lives in `src/`, broken down by responsibility: `main.rs` orchestrates CLI flow, `formatter.rs` holds the core formatting rules, `config.rs` reads `.editorconfig`, `ignores.rs` handles skip logic, and `treesitter.rs` hosts parsing glue. Reference fixtures sit in `samples/`, with expected formatter output in `results/`. Build artifacts land in `target/`. Keep new utilities under `src/` or a dedicated module folder, and place large shared helpers in their own module to keep `main.rs` lean.

## Build, Test, and Development Commands
- `cargo build` compiles the crate in debug mode; use `cargo build --release` or `make build` before distributing binaries.
- `cargo run -- samples/ --output results/` executes the formatter against the sample corpus; `make run` mirrors this flow.
- `cargo fmt` and `cargo clippy` keep code aligned with `rustfmt.toml` and lint expectations. Run `cargo fmt -- --check` in CI contexts.

## Coding Style & Naming Conventions
Rust code is formatted with tabs (`hard_tabs = true`) and a wide `max_width`. Always run `cargo fmt` before pushing. Brace placement follows `AlwaysNextLine`, so opening braces should sit on their own lines. Module and file names stay snake_case; types use UpperCamelCase; functions and variables use snake_case. Keep public APIs documented with `///` docs when they leave `src/main.rs`.

## Testing Guidelines
Unit and integration tests belong under `tests/` or alongside modules in `src/`. Prefer precise fixture-based assertions to exercise formatter output—for example, compare against files in `samples/` and write expected results to a temp dir. Run `cargo test` before every PR; extend coverage when touching parsing or config logic. Name tests with the behavior under test, such as `formats_nested_switches`.

## Commit & Pull Request Guidelines
Follow the existing history: concise, present-tense summaries in lowercase (`update readme`, `cleaned up some warnings`). Group related changes per commit and include rationale in the body when behavior shifts. Pull requests should outline the problem, the approach, and testing evidence (commands run, fixtures added). Link to issues when applicable and attach before/after code snippets for formatter regressions. Request review once CI (build, fmt, clippy, test) is green.

## Release & Packaging Notes
`Makefile` targets `run`, `run_new`, and `all` are tuned for generating release binaries via Docker for macOS and Linux. If you add a new platform or distribution step, follow this pattern and keep artifacts under `target/release/platforms/`.
