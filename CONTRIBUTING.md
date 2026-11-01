# Contributing to EscapeTypo

Thank you for your interest in improving EscapeTypo.

## Before you start

Please check the existing issues and pull requests before starting work. For larger changes, open an issue first so the approach can be discussed.

## Development setup

You will need:

- Rust stable
- Cargo
- A terminal
- A PDF with selectable text for manual testing

Clone the repository and enter the project directory:

```bash
git clone https://github.com/Dhruv-Creator-De-Pal/EscapeTypo.git
cd EscapeTypo
```

Run the application during development:

```bash
cargo run
```

## Before opening a pull request

Run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
cargo test --all-targets
```

Please make sure all checks pass before submitting a pull request.

## Pull requests

A pull request should:

- Clearly describe the change
- Explain how it was tested
- Keep unrelated changes out of the pull request
- Update the README when user-facing behavior changes
- Include screenshots or a short recording for visible interface changes

## Issues

When reporting a problem, include:

- Operating system
- Rust version
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Relevant error output

## Code style

Follow standard Rust formatting and prefer clear, focused changes. Avoid committing generated files, personal progress files, build directories, or secrets.

## Project status

The timer experience is still being refined. Suggestions and open-source contributions are welcome.
