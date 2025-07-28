# tinygrep

A lightweight command-line tool for searching through files to find instances of a given query string.

## Features

- Fast text searching within files
- Case-sensitive matching
- Simple command-line interface

## Usage

```bash
tinygrep <query> <filename> [sen|ins]
```

## Example

```bash
tinygrep hello example.txt
```

This will search for all instances of "hello" in `example.txt` and display the matching lines.

## Case Sensitivity

tinygrep performs case-sensitive matching by default. For example:

```bash
tinygrep hello example.txt sen
```

This will match "hello" but not "Hello" or "HELLO".

```bash
tinygrep hello example.txt ins
```

This will match "hello" and "Hello" or "HELLO".