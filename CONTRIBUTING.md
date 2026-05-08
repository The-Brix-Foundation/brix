# Contributing to Brix

Thank you for your interest in contributing to Brix! Every contribution helps, whether it's fixing a typo, reporting a bug, or implementing a new compiler feature.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/brix.git`
3. Create a branch: `git checkout -b my-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Run the formatter: `cargo fmt`
7. Run clippy: `cargo clippy -- -D warnings`
8. Commit your changes with a clear message
9. Push to your fork and open a Pull Request

## Development Setup

You'll need:

- Rust (stable, latest) — install via [rustup](https://rustup.rs/)
- LLVM 17+ — for code generation
- A C linker (gcc or clang)

Build the project:

```bash
cargo build
```

Run the test suite:

```bash
cargo test
```

## Code Style

- Run `cargo fmt` before committing — all code must be formatted
- Run `cargo clippy` — no warnings allowed
- Write tests for new features
- Keep functions small and focused
- Add comments for non-obvious logic

## Commit Messages

Write clear, concise commit messages. Use the present tense:

- `Add token types for string literals`
- `Fix off-by-one error in lexer span tracking`
- `Implement if/else parsing`

Avoid generic messages like "fix stuff" or "update code".

## Pull Requests

- Keep PRs focused — one feature or fix per PR
- Write a clear description of what changed and why
- Reference any related issues (e.g., "Closes #42")
- Make sure CI passes before requesting review
- Be open to feedback — we review to improve, not to gatekeep

## Reporting Bugs

Open an issue with:

- A clear title describing the problem
- Steps to reproduce
- Expected behavior vs actual behavior
- Your environment (OS, Rust version, LLVM version)

## Feature Requests

Open an issue tagged with `feature` and describe:

- What the feature does
- Why it's useful
- How it might work (if you have ideas)

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold a welcoming and respectful environment.

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache 2.0, as described in the [README](README.md#license).
