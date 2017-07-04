# vanityhash
> Find your perfect hash

```sh
vanityhash 0.2
Evan Schwartz <evan@ripple.com>
Searches for hashes that match a certain prefix

USAGE:
    vanityhash [FLAGS] [OPTIONS] <prefix>

FLAGS:
    -h, --help                  Prints help information
        --include_both_cases    Include upper and lowercase letters
        --include_symbols       Include lookalike characters and numbers
    -V, --version               Prints version information

OPTIONS:
        --preimage_prefix <PREIMAGE_PREFIX>    The prefix the hash preimage should start with
    -t, --threads <NUM_THREADS>                Number of threads to use [default: 8]

ARGS:
    <prefix>    The prefix the vanity hash should start with

```
