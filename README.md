# `dedup`

A program that locates duplicate image files in a directory by comparing SHA256
hashes. This program is similar to `fdupes` (see
[fdupes(1)](https://linux.die.net/man/1/fdupes)), but I needed a customized
option for deduplicating large amounts of AI slop.

This program does not recursively scan subdirectories. This program uses file
extensions to determine whether files are images, as oppsed to attempting to
parse EXIF metadata.

## Table of Contents

- [Build](#build)
- [Usage](#usage)
  - [Delete duplicates (default)](#delete-duplicates-default)
  - [Preview duplicates without deleting](#preview-duplicates-without-deleting)
- [License](#license)

## Build
Run:
```
cargo install --path . --root ~/.local
```
Which will build a release binary and install it under `~/.local/bin`.

## Usage

### Delete duplicates (default)
Run:
```bash
dedup /path/to/directory
```
Or just:
```bash
dedup
```
To scan the current directory. This will locate the duplicates and
interactively ask which file should be kept, then delete the duplicates.

### Preview duplicates without deleting
To preview duplicates without deleting them, use the `--dry-run` flag:
```bash
dedup /path/to/directory --dry-run
```
This will locate and print the duplicates without deleting anything.

## License

MIT License — see [`LICENSE`](LICENSE) for details.
