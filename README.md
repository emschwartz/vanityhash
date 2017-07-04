# vanityhash
> Find your perfect hash

```sh
vanityhash 0.1
Evan Schwartz <evan@ripple.com>
Searches for hashes that match a certain prefix

USAGE:
    vanityhash [OPTIONS] <prefix>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --include_both_cases <INCLUDE_BOTH_CASES>    Include upper and lowercase letters
        --include_symbols <INCLUDE_SYMBOLS>          Include lookalike characters and numbers
        --preimage_prefix <PREIMAGE_PREFIX>          The prefix the hash preimage should start with
    -t, --threads <NUM_THREADS>                      Number of threads to use [default: 8]

ARGS:
    <prefix>    The prefix the vanity hash should start with
```
