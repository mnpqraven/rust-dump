```
rsync-tool 0.1.1

USAGE:
    rsync-tool [OPTIONS] <LOCAL> <HOST>

ARGS:
    <LOCAL>    local file/folder path to copy
    <HOST>     nas address

OPTIONS:
    -h, --help                      Print help information
    -p <PORT>                       ssh port [default: 22]
    -r, --rev                       reverse data flow (copy data from remote address to local
                                    directory)
    -V, --version                   Print version information
        --var <use-var-services>    use /var/services directory tree instead of /volume1 [default:
                                    false]
```
