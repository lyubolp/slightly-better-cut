# slightly-better-cut

A slightly better implementation of the cut command in Linux with Python-style indexing.


[![Tests](https://github.com/lyubolp/slightly-better-cut/actions/workflows/tests.yml/badge.svg)](https://github.com/lyubolp/slightly-better-cut/actions/workflows/tests.yml)[![GPL-3.0](https://img.shields.io/badge/license-GPL_3.0-blue.svg)](https://github.com/lyubolp/slightly-better-cut/blob/main/LICENSE)

<!--[![Crates.io](https://img.shields.io/crates/v/broot.svg)](https://crates.io/crates/broot)-->

## Install

### From crates.io

TBD

### From binary

TBD

### From source

0. Install [Rust](https://www.rust-lang.org/tools/install)
1. Clone the repository - `git clone https://github.com/lyubolp/slightly-better-cut`
2. Build the code - `cargo build -r`
3. Resulting binary is located under `<repo_dir>/target/release/sbcut`

## Synopsis

```bash
Usage: sbcut [OPTIONS] <--bytes <LIST>|--characters <LIST>|--fields <LIST>> [FILE]

Arguments:
  [FILE]  [default: -]

Options:
  -b, --bytes <LIST>
  -c, --characters <LIST>
  -d, --delimiter <DELIM>               [default: "\t"]
  -f, --fields <LIST>
      --complement
  -s, --only_delimited
      --always_show_no_delimited_lines
      --output_delimiter <DELIM>
  -z, --zero_terminated
  -h, --help                            Print help
  -V, --version                         Print version
```

## Description

Print selected parts of lines from each file to standard output.

With no FILE or when FILE is `-` , read from standard input.

Options:

- `-b`, `--bytes=LIST` - select only those bytes
- `-c`, `--characters=LIST` - select only these characters
- `-d`, `--delimiter=DELIM` - use DELIM instead of TAB for field delimiter
- `-f`, `--fields=LIST` - select only these fields; also print any line that contains no delimited character, unless the -s option is specified
- `--complement` - complement the set of selected bytes, characters or fields
- `-s`, `--only-delimited` - do not print lines not containing delimiters
- `--output-delimiter=STRING` - use STRING as the output delimiter the default is to use the input delimiter
- `-z`, `--zero-terminated` - line delimiter is NUL, not newline
- `--help` display this help and exit
- `--version` - output version information and exit

Use one, and only one of -b, -c or -f. Each LIST is made up of one range, or many ranges separated by commas.
Selected input is written in the same order that it is read, and is written exactly once. Each range is one of:

`N:M:S` - `N` is start, `M` is end, `S` is step.

Defaults - `N` = 1, `M` = last character of current line, `S` = 1. `N < M` !

Return all bytes, characters or fields in the range, defined by `N`, `M` and `S`

## Usage

Given the following file:

```bash
ID,Name,Age,Email,City,Country,Occupation,Salary
1,John Doe,28,john.doe@example.com,New York,USA,Software Engineer,80000
2,Jane Smith,34,jane.smith@example.com,Los Angeles,USA,Data Analyst,75000
3,Bob Johnson,45,bob.johnson@example.com,Chicago,USA,Project Manager,90000
```

We can extract only the first column:

(Fields are zero-indexed)

```bash
$ sbcut -d "," -f0 sample_bigger.csv
ID
1
2
3
```

The last two:

```bash
$ sbcut -d "," -f-2: sample_bigger.csv
Occupation,Salary
Software Engineer,80000
Data Analyst,75000
Project Manager,90000
```

First column, third to fith and last two:

```bash
$ sbcut -d "," -f0,2:4,-2: sample_bigger.csv
ID,Age,Email,Occupation,Salary
1,28,john.doe@example.com,Software Engineer,80000
2,34,jane.smith@example.com,Data Analyst,75000
3,45,bob.johnson@example.com,Project Manager,90000
```

Every even-numbered column:

```bash
$ $sbcut -d "," -f::2 sample_bigger.csv
ID,Age,City,Occupation
1,28,New York,Software Engineer
2,34,Los Angeles,Data Analyst
3,45,Chicago,Project Manager
```

## Documentation

TBD

## Coreutils cut compatibility

- Indexing
- `always_show_no_delimited_lines`
- `-z` line ending

## Contributing

TBD

## Licence

[GPL-3.0](https://choosealicense.com/licenses/gpl-3.0/)
