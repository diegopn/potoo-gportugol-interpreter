# Potoo G-Portugol Interpreter

An independent Rust interpreter library for the G-Portugol 1.4.0 language. G-Portugol source files use the `.gpt` extension and remain separate from Portugol 2.0 (`.por`), VisuAlg (`.alg`), and Potigol (`.poti`). The crate can be embedded in Rust applications; it includes a standalone terminal command-line runner.

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

The current releases target Linux x86_64 (GNU), Windows x86_64 (MSVC), and macOS Intel (x86_64). Apple silicon (arm64) is not supported in this version.

## Development

Run these commands from the repository root:

```sh
cargo fmt --all -- --check
cargo check
cargo test
```

## License

This crate is licensed under GPL-3.0-or-later. It is an independent implementation: it neither invokes nor bundles the upstream `gpt` executable and does not reuse upstream implementation code.

## Project policies
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## Standalone command-line runner

The `potoo-gportugol` command runs one source file without Potoo IDE. For example:

```sh
potoo-gportugol example.gpt
```

The terminal runner provides standard input/output and does not provide an interactive debugger or module selection for external source files.

## Install a prebuilt runner

Download the archive for your system and `SHA256SUMS` from the [latest GitHub Release](https://github.com/diegopn/potoo-gportugol-interpreter/releases/latest). Binary releases are not published yet. The planned first release will be tagged `v0.1.0`; its release workflow is configured to produce these asset names:

| System | Archive |
| --- | --- |
| Linux x86_64 | `potoo-gportugol-interpreter-v0.1.0-x86_64-unknown-linux-gnu.tar.gz` |
| Windows x86_64 | `potoo-gportugol-interpreter-v0.1.0-x86_64-pc-windows-msvc.zip` |
| macOS Intel | `potoo-gportugol-interpreter-v0.1.0-x86_64-apple-darwin.tar.gz` |


Current CI and release targets are Linux x86_64, Windows x86_64, and macOS Intel (x86_64). Apple silicon (arm64) is not supported in this release; support may be added in a future version.

### Linux x86_64

```sh
asset=potoo-gportugol-interpreter-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
release=https://github.com/diegopn/potoo-gportugol-interpreter/releases/download/v0.1.0
curl -fLO "$release/$asset"
curl -fLO "$release/SHA256SUMS"
sha256sum "$asset" # compare with its line in SHA256SUMS
tar -xzf "$asset"
mkdir -p "$HOME/.local/bin"
install -m 755 "potoo-gportugol-interpreter-v0.1.0-x86_64-unknown-linux-gnu/potoo-gportugol" "$HOME/.local/bin/potoo-gportugol"
potoo-gportugol example.gpt
```

Add it for the current shell with `export PATH="$HOME/.local/bin:$PATH"`; place that line in your shell profile to keep it for future sessions.

### Windows x86_64 (PowerShell)

```powershell
$asset = "potoo-gportugol-interpreter-v0.1.0-x86_64-pc-windows-msvc.zip"
$release = "https://github.com/diegopn/potoo-gportugol-interpreter/releases/download/v0.1.0"
Invoke-WebRequest "$release/$asset" -OutFile $asset
Invoke-WebRequest "$release/SHA256SUMS" -OutFile SHA256SUMS
Get-FileHash ".\$asset" -Algorithm SHA256 # compare with the matching SHA256SUMS entry
Expand-Archive ".\$asset" -DestinationPath .\potoo-gportugol-interpreter
.\potoo-gportugol-interpreter\potoo-gportugol-interpreter-v0.1.0-x86_64-pc-windows-msvc\potoo-gportugol.exe .\example.gpt
```

To run `potoo-gportugol.exe` from any folder, add its extracted directory to your user `PATH` in Windows Environment Variables.

### macOS Intel

This release supports Intel Macs only. Apple silicon is not supported in this version.

```sh
asset=potoo-gportugol-interpreter-v0.1.0-x86_64-apple-darwin.tar.gz
release=https://github.com/diegopn/potoo-gportugol-interpreter/releases/download/v0.1.0
curl -fLO "$release/$asset"
curl -fLO "$release/SHA256SUMS"
shasum -a 256 "$asset" # compare with its line in SHA256SUMS
tar -xzf "$asset"
mkdir -p "$HOME/.local/bin"
install -m 755 "${asset%.tar.gz}/potoo-gportugol" "$HOME/.local/bin/potoo-gportugol"
potoo-gportugol example.gpt
```

Add it for the current shell with `export PATH="$HOME/.local/bin:$PATH"`; place that line in `~/.zprofile` to keep it for future macOS sessions.

## Install from source

With Rust 1.93 or later and network access, Cargo can install the command directly from its tagged source:

```sh
cargo install --git https://github.com/diegopn/potoo-gportugol-interpreter.git --rev 795e262f4ac160b720266e149d8fc3e2cd916a1c --bin potoo-gportugol
```

## Use as a Rust library

```toml
[dependencies]
potoo-gportugol-interpreter = { git = "https://github.com/diegopn/potoo-gportugol-interpreter.git", rev = "795e262f4ac160b720266e149d8fc3e2cd916a1c" }
```

The library API lets Rust applications provide their own `RuntimeIo` implementation.

## DOI

A stable Figshare DOI will be added when the first public release is published.
