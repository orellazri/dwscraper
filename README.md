<div align="center">

# dwscraper

This is a scraper for the Digital Whisper website which allows you to download issues.

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

Download issue #5:

```bash
dwscraper download 5
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
