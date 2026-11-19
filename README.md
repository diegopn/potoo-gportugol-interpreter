<p align="center">
  <img src="https://diegopn.github.io/potoo-site/src/assets/images/language-interpreter-icon.svg?v=1c651ca7" alt="Potoo IDE language interpreter icon" width="120" />
</p>

<h1 align="center">Potoo G-Portugol Interpreter</h1>

<p align="center">An independent Rust interpreter library and CLI for G-Portugol 1.4.0.</p>

<p align="center">
  <a href="https://github.com/diegopn/potoo-gportugol-interpreter/releases/latest">Download</a> ·
  <a href="https://diegopn.github.io/potoo-site/">Website</a> ·
  <a href="https://diegopn.github.io/potoo-site/manual/g-portugol/">User manual</a>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a href="https://doc.rust-lang.org/cargo/"><img alt="Cargo" src="https://img.shields.io/badge/Cargo-DEA584?style=for-the-badge&logo=rust&logoColor=white"></a>
  <img alt="Rust library" src="https://img.shields.io/badge/Rust_library-4A86CF?style=for-the-badge">
  <img alt="CLI" src="https://img.shields.io/badge/CLI-6A4C93?style=for-the-badge">
</p>

Originally developed as part of the Potoo IDE project, this interpreter is an independent Rust software package intended to be distributed separately from the IDE. It can be embedded as a library in other Rust projects or run as a standalone command-line tool on Linux, macOS, and Windows.

G-Portugol source files use `.gpt` and remain separate from Portugol 2.0 (`.por`), VisuAlg (`.alg`), and Potigol (`.poti`). The crate can be embedded in Rust applications.

## Original language references

- [G-Portugol project site and online manual](https://gportugol.github.io/)
- [Upstream GPT compiler and interpreter](https://github.com/gportugol/gpt)

These are the original language and implementation references for compatibility work. This repository provides an independent Rust implementation.

## Original language references

- [G-Portugol project site and online manual](https://gportugol.github.io/)
- [Upstream GPT compiler and interpreter](https://github.com/gportugol/gpt)

These are the original language and implementation references for compatibility work. This repository provides an independent Rust implementation.

## Language coverage

The implementation follows the G-Portugol 1.4.0 manual and grammar for declarations, primitive types, expressions and conversions, integer division, `imprima`, typed `leia()`, conditionals, loops, vectors, matrices, and functions. It provides lexer, parser, type checking, runtime diagnostics with source locations, execution limits, cancellation, host-controlled input/output, and debugger events. The language defines `imprima` and `leia()` as intrinsic functions; Potoo also embeds an independent Rust implementation of the default `base.gpt` math library (`absoluto`, `arredonda`, `potencia`, and `raiz_quadrada`).

G-Portugol keywords may contain accents, while identifiers are ASCII. Vector and matrix indexes start at zero; arrays are initialized with zero or a null value. Arrays passed to functions are copied by value. Nested functions and reference parameters are not supported by the language.

Integer storage and integer operations follow GPT 1.4.0's signed 32-bit behavior. Overflow wraps; `-2147483648 / -1` returns `-2147483648`, and its remainder is zero. Mixed expressions still promote operands to real before arithmetic when a real operand is present.

## Source example

```gpt
algoritmo votacao;
variáveis idade: inteiro; fim-variáveis
início
  imprima("Qual é sua idade?");
  idade := leia();
  se idade >= 18 então
    imprima("Pode votar.");
  senão
    imprima("Ainda não pode votar.");
  fim-se
fim
```

This example is covered by the interpreter's tests in `tests/`.

## Rust API

The crate has no third-party dependencies. Add the pinned Git dependency below to your Rust application's `Cargo.toml`.

```toml
[dependencies]
potoo-gportugol-interpreter = { git = "https://github.com/diegopn/potoo-gportugol-interpreter.git", rev = "795e262f4ac160b720266e149d8fc3e2cd916a1c" }
```

The main APIs are:

- `lex(source)` returns tokens with source spans.
- `parse(source)` and `parse_with_options(source, options)` build the public AST.
- `check(&program)` performs static type checking.
- `execute(source, io)` parses, checks, and runs a program, returning `Some(code)` when the main block executes `retorne <inteiro>` and `None` when it ends normally.
- `execute_with_options(source, io, options)` returns the same optional main-block status with caller-configured limits.
- `execute_with_sources(main, modules, io)` composes a main file with function-only source files and returns its optional status.
- `execute_with_sources_and_options(main, modules, io, options)` does the same with caller-configured limits.

The host implements `RuntimeIo` to receive output, provide typed input, signal cancellation, and handle debugger events. This keeps terminal, desktop, or other host I/O outside the interpreter and lets the IDE run it on its existing execution worker.

Default resource limits are 1,000,000 execution steps, 1 MiB of output, 1 MiB per string, 100,000 array elements, 32 nested function calls, 128 parser nesting levels, and a 30-second input deadline. The IDE host enforces the input deadline. Other `RuntimeIo` hosts must override `read_line_with_timeout` to enforce it; the default implementation preserves the host's existing blocking `read_line` behavior. Hosts can override the values through `ExecutionOptions`.

## Compatibility and platform status

The documented 1.4.0 language is the compatibility target. The upstream `gpt` executable was not available for a differential run, so compatibility has been checked with the crate's local lexer, parser, type, runtime, I/O, limits, and debugger tests rather than compared against the reference executable.

The crate API supports GPT's multi-file source composition when the host supplies the main file and its function-only source files. The IDE currently sends the active `.gpt` document as a single source; it does not yet provide project-level module selection or source-aware navigation for external functions.

The v0.1.1 release provides native prebuilt packages for Linux x86_64 (GNU), Windows x86_64 (MSVC), Intel macOS x86_64, and Apple silicon macOS arm64.

## Development

Run these commands from the repository root:

```sh
cargo fmt --all -- --check
cargo check
cargo test
```

## License

This crate is licensed under GPL-3.0-or-later. It is an independent implementation: it neither invokes nor bundles the upstream `gpt` executable and does not reuse upstream implementation code.

## Support

If this interpreter is useful in your work, you can support its continued development:

<p align="left">
  <a href="https://github.com/sponsors/diegopn"><img alt="Become a sponsor" src="https://img.shields.io/badge/Become_a_sponsor-EA4AAA?style=for-the-badge&logo=githubsponsors&logoColor=white"></a>
</p>

## Project policies

- [Code of conduct](CODE_OF_CONDUCT.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## Standalone command-line runner

The `potoo-gportugol` command runs one source file without Potoo IDE. Save this program as `example.gpt`:

```gpt
algoritmo exemplo; início imprima("Olá, G-Portugol!"); fim
```

Run it with:

```sh
potoo-gportugol example.gpt
```

The program prints `Olá, G-Portugol!`. The terminal runner provides standard input/output and does not provide an interactive debugger or module selection for external source files.

## Install a prebuilt runner

The v0.1.1 release provides native CLI packages for Linux x86_64 (GNU), Windows x86_64 (MSVC), Intel macOS x86_64, and Apple silicon macOS arm64. Download your platform's archive and `SHA256SUMS` from the [latest release](https://github.com/diegopn/potoo-gportugol-interpreter/releases), then run the commands below from the folder containing both files. Each archive contains `potoo-gportugol` (or `potoo-gportugol.exe` on Windows) and the repository license. Replace `example.gpt` with your source file; the release package does not include a sample file.

The terminal runner executes one source file with standard input/output. It does not provide an interactive debugger or module selection for external source files.

### Linux x86_64 (GNU)

```sh
set -euo pipefail
asset="potoo-gportugol-interpreter-v0.1.1-x86_64-unknown-linux-gnu.tar.gz"
grep -F "  $asset" SHA256SUMS | sha256sum --check -
tar -xzf "$asset"
mkdir -p "$HOME/.local/bin"
install -m 755 "${asset%.tar.gz}/potoo-gportugol" "$HOME/.local/bin/potoo-gportugol"
export PATH="$HOME/.local/bin:$PATH"
potoo-gportugol example.gpt
```

To keep the command available in new Linux sessions, add the `export PATH="$HOME/.local/bin:$PATH"` line to your shell profile.

### Windows x86_64 (MSVC, PowerShell)

```powershell
$archive = "potoo-gportugol-interpreter-v0.1.1-x86_64-pc-windows-msvc.zip"
$line = Get-Content .\SHA256SUMS | Where-Object { $_.EndsWith("  $archive") } | Select-Object -First 1
if (-not $line) { throw "No SHA256SUMS entry found for $archive" }
$expected = ($line -split '\s+')[0]
$actual = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) { throw "SHA-256 mismatch for $archive" }
$installDir = Join-Path $env:USERPROFILE ".local\bin"
New-Item -ItemType Directory -Force $installDir | Out-Null
Expand-Archive -LiteralPath $archive -DestinationPath $installDir
$exe = Join-Path $installDir "potoo-gportugol-interpreter-v0.1.1-x86_64-pc-windows-msvc\potoo-gportugol.exe"
& $exe .\example.gpt
```

To run `potoo-gportugol.exe` from any folder, add `$env:USERPROFILE\.local\bin\potoo-gportugol-interpreter-v0.1.1-x86_64-pc-windows-msvc` to your user `Path` environment variable in Windows Settings.

### Intel macOS x86_64

```sh
set -euo pipefail
asset="potoo-gportugol-interpreter-v0.1.1-x86_64-apple-darwin.tar.gz"
grep -F "  $asset" SHA256SUMS | shasum -a 256 -c -
tar -xzf "$asset"
mkdir -p "$HOME/.local/bin"
install -m 755 "${asset%.tar.gz}/potoo-gportugol" "$HOME/.local/bin/potoo-gportugol"
export PATH="$HOME/.local/bin:$PATH"
potoo-gportugol example.gpt
```

### Apple silicon macOS arm64

```sh
set -euo pipefail
asset="potoo-gportugol-interpreter-v0.1.1-aarch64-apple-darwin.tar.gz"
grep -F "  $asset" SHA256SUMS | shasum -a 256 -c -
tar -xzf "$asset"
mkdir -p "$HOME/.local/bin"
install -m 755 "${asset%.tar.gz}/potoo-gportugol" "$HOME/.local/bin/potoo-gportugol"
export PATH="$HOME/.local/bin:$PATH"
potoo-gportugol example.gpt
```

On macOS, add the `export PATH="$HOME/.local/bin:$PATH"` line to `~/.zprofile` to keep the command available in future sessions.

## Install from source

With Rust 1.93 or later and network access, Cargo can install the command directly from its tagged source:

```sh
cargo install --git https://github.com/diegopn/potoo-gportugol-interpreter.git --tag v0.1.1 --bin potoo-gportugol
```

## Use as a Rust library

```toml
[dependencies]
potoo-gportugol-interpreter = { git = "https://github.com/diegopn/potoo-gportugol-interpreter.git", rev = "795e262f4ac160b720266e149d8fc3e2cd916a1c" }
```

The library API lets Rust applications provide their own `RuntimeIo` implementation.

## DOI and citation

DOI: [10.6084/m9.figshare.34914234](https://doi.org/10.6084/m9.figshare.34914234)

Suggested reference (ABNT NBR 6023:2018):

> NASCIMENTO, Diego Pereira do. *G-Portugol interpreter v0.1.0*. 2026. Programa de computador. Disponível em: https://doi.org/10.6084/m9.figshare.34914234. Acesso em: 4 nov. 2026.
