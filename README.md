# Performance Sparking Hub (PSH)

[TODO]

## Contribution Guide

Before submitting a pull request (PR) to PSH, it's crucial to perform a self-check to ensure the quality and adherence to coding standards. Follow these steps for an effective self-check:

- Run Clippy:
Execute `cargo clippy`, a lint tool for Rust designed to catch common mistakes and enhance the overall quality of your Rust code.

- Format Code:
Utilize `cargo fmt` to format your Rust code, ensuring consistency in code formatting across the project.

- Security Audit:
Employ `cargo audit` to enhance the security of your Rust code. This command reviews your dependencies for any security vulnerabilities reported to the RustSec Advisory Database. If you haven't installed `cargo-audit` yet, you can do so by running `cargo install cargo-audit`.

Failing to adhere to these self-check steps might result in your PR not being reviewed promptly. Without completing these checks, the chances of finding a reviewer willing to assess your PR may be reduced. Therefore, it is essential to diligently follow the outlined steps to increase the likelihood of a successful and timely review for your pull request.
