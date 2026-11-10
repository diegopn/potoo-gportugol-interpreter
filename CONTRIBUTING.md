# Contributing

Contributions to G-Portugol are welcome, including bug reports, interpreter tests, documentation, and code. By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md). For vulnerabilities, follow [SECURITY.md](SECURITY.md) and do not report them publicly.

## Report a bug

Open an [issue](https://github.com/diegopn/potoo-gportugol-interpreter/issues) with the crate version or commit, Rust version, operating system, a small `.gpt` example, expected and actual behavior, and steps to reproduce the issue. Use a minimal example and remove credentials, personal information, and unrelated project data.

## Development setup

Install Rust 1.93 or later. From the repository root, run:

```sh
cargo fmt --all -- --check
cargo check
cargo test
cargo package
```

## Pull requests

- Target `develop` for ongoing development; `main` is reserved for stable releases.
- Keep each pull request focused and describe the change and how to review it.
- List the verification commands you ran.
- Preserve copyright and license notices. Check license compatibility for third-party material and disclose added dependencies with their purpose and license.

## Changes to language behavior

- Add tests for new syntax, runtime behavior, diagnostics, and resource limits where applicable.
- Keep the documented G-Portugol 1.4.0 compatibility target explicit. Do not claim behavior that has not been implemented and verified.
- Keep public documentation in English and update it when the API or supported behavior changes.

## License for contributions

Contributions are distributed under the GNU General Public License version 3 or, at your option, any later version. See [LICENSE](LICENSE).
