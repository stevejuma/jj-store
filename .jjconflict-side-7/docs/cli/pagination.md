# Output and paging

Custom help and text results request a pager only on a terminal. `--no-pager`
disables paging. `JJ_STORE_PAGER` selects a pager command; `cat` disables it.
The default is `less -FRX`. The value is an executable with optional quoted
arguments; it is not evaluated by a shell. Missing pagers warn and fall back to
stdout. JSON is newline-terminated, plain and never paged. `--color auto`
requires a terminal and an unset or empty `NO_COLOR`; `always` and `never`
override that.
