<div align="center">

# dwscraper

[![Rust](https://github.com/orellazri/dwscraper/actions/workflows/test.yml/badge.svg)](https://github.com/orellazri/dwscraper/actions/workflows/test.yml)

This is a scraper for the [Digital Whisper](https://digitalwhisper.co.il) website. It allows you to download issues.

</div>

## Build

1. Clone the repository
2. Run:

```bash
cargo build
```

## Usage

```bash
dwscraper --help
```

## Examples

### Download

Download issue #5:

```bash
dwscraper download 5
```

Download to a specific directory:

```bash
dwscraper -o ~/issues download 5
```

Download issues 4,5,6,7:

```bash
dwscraper download 4:7
```

Download all issues:

```bash
dwscraper download :
```

Download issues 100 and above:

```bash
dwscraper download 100:
```

Download all issues under 50:

```bash
dwscraper download :50
```

### Archive

Archive all issues:

```bash
dwscraper -o ~/documents archive
```

_(This will download all missing issues that don't exist the output directory)_
