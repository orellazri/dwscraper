<div align="center">

# dwscraper

[![Tests](https://github.com/orellazri/dwscraper/actions/workflows/tests.yml/badge.svg)](https://github.com/orellazri/dwscraper/actions/workflows/tests.yml)

This is a scraper for the [Digital Whisper](https://digitalwhisper.co.il) website. It allows you to download issues.

</div>

It uses Rayon for data parallelism, making it extremely fast - capabale of downloading an issue in an average of <100ms (when downloading a large amount - e.g. archiving)

## Download
You can download the latest release for linux from the [releases page](https://github.com/orellazri/dwscraper/releases).

For other platforms, you can build the binary from the source code.

## Build

1. Clone the repository
2. Run:

```bash
cargo build --release
```

3. By default, the resulting binary will be located in `target/release/dwscraper`

## Usage

```bash
Usage: dwscraper [OPTIONS] <COMMAND>

Commands:
  download  Download issues
  archive
  help      Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>  Output directory
  -h, --help             Print help
  -V, --version          Print version
```

_**Note:** By default, the output directory is the current directory. You can change that with the `-o` flag_

`download`: Specify an issue number, a range (`start:finish`, inclusive), or `last` for last issue

For example:

- `5` - Issue #5
- `7:10` - Issues 7,8,9,10
- `:` - All issues
- `:10` - First 10 issues
- `20:` - All issues above #20
- `last` - Last issue

`archive`: Download all missing issues in the output directory
