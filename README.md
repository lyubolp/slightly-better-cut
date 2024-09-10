# slightly-better-cut
A slightly better implementation of the cut command in Linux.

## Synopsis
`sbcut OPTION ... [FILE] ..`

## Description
Print selected parts of lines from each file to standard output.

With no FILE or when FILE is `-` , read from standard input.

Options:

- `-b`, `--bytes=LIST` - select only those bytes
- `-c`, `--characters=LIST` - select only these characters
- `-d`, `--delimiter=DELIM`  - use DELIM instead of TAB for field delimiter
- `-f`, `--fields=LIST` - select only these fields; also print any line that contains no delimited character, unless the -s option is specified
- `--complement` - complement the set of selected bytes, characters or fields
- `-s`, `--only-delimited` - do not print lines not containing delimiters
- `--output-delimiter=STRING` - use STRING as the output delimiter the default is to use the input delimiter
- `-z`, `--zero-terminated` - line delimiter is NUL, not newline
- `--help` display this help and exit
- `--version` - output version information and exit

Use one, and only one of -b, -c or -f.  Each LIST is made up of one range, or many ranges separated by commas.
Selected  input  is written in the same order that it is read, and is written exactly once.  Each range is one of:

`N:M:S` - `N` is start, `M` is end, `S` is step. 

Defaults - `N` = 1, `M` = last character, `S` = 1

Return all bytes, characters or fields in the range, defined by `N`, `M` and `S`

