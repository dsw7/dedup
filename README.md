# File Deduplicator

A program that locates duplicate image files in a directory by comparing SHA256
hashes. This program is similar to `fdupes` (see
[fdupes(1)](https://linux.die.net/man/1/fdupes)), but I needed a customized
option for deduplicating large amounts of AI slop.

This program does not recursively scan subdirectories. This program uses file
extensions to determine whether files are images, as oppsed to attempting to
parse EXIF metadata.

## Table of Contents

- [Usage](#usage)
  - [List but do not delete duplicates](#list-but-do-not-delete-duplicates)
  - [List and delete duplicates](#list-and-delete-duplicates)

## Usage

### List but do not delete duplicates
This is the default. Run
```bash
dedup /path/to/directory
```
Or just:
```bash
dedup
```
To scan the current directory.

### List and delete duplicates
Run:
```bash
dedup /path/to/directory --delete # or -d
```
This will locate the duplicates and interactively ask which file should be
kept.
