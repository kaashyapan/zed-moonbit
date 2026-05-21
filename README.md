# zed-moonbit

[MoonBit](https://www.moonbitlang.com) language support for the [Zed](https://zed.dev) editor.

## Features

- Syntax highlighting via `tree-sitter-moonbit`
- Code outline (functions, types, traits)
- Bracket matching and auto-indent
- Full LSP support via `moon-lsp` (bundled with the MoonBit toolchain):
  - Completions
  - Diagnostics / type errors
  - Go-to-definition
  - Hover types & docs
  - Find references
  - Rename

## Prerequisites

Install the **MoonBit toolchain**.  The install script places all binaries —
including `moon-lsp` — at `~/.moon/bin/`:

```bash
# macOS / Linux
curl -fsSL https://cli.moonbitlang.com/install/unix.sh | bash

# Windows (PowerShell)
irm https://cli.moonbitlang.com/install/powershell.ps1 | iex
```

After installation, verify the LSP binary exists:

```bash
~/.moon/bin/moon-lsp --version
```

Keep the toolchain up-to-date with:

```bash
moon upgrade
```

## Installation

### From the Zed extension store (once published)

Open the Extensions panel (`cmd-shift-x` / `ctrl-shift-x`), search for
**MoonBit**, and click **Install**.

### Local / development install

```bash
git clone https://github.com/your-name/zed-moonbit
```

Then in Zed: **Extensions → Install Dev Extension** → select the cloned folder.


If `moon-lsp` is not on your `$PATH`, the extension automatically falls
back to `~/.moon/bin/moon-lsp` (macOS/Linux) or
`%APPDATA%\moon\bin\moon-lsp.exe` (Windows).


## Building

```bash
cargo build --release --target wasm32-wasip1
```

Zed handles this automatically when you use **Install Dev Extension**.

## License

MIT
