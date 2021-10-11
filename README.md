Yes, I know about GPU generators.

https://smith-mcf.medium.com/solana-vanity-address-using-gpus-5a68ad94d1d4

```bash
./solana-nice-address --help
solana-nice-address 0.1.0
Generate Nice Solana Address By Template

USAGE:
    solana-nice-address [FLAGS] [OPTIONS] <word>

FLAGS:
    -e, --exit           Exit on first match
    -h, --help           Prints help information
    -i, --ignore-case    Ignore case distinctions
    -V, --version        Prints version information

OPTIONS:
    -s, --stat <stat>          Print genrate every seconds
    -t, --threads <threads>    Number of threads for lookup [default: 8]

ARGS:
    <word>    Filter by starting from word
```
