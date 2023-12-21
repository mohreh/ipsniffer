run `cargo run -- -h` for help.

```sh
Usage: ipsniffet [-a=ARG] [-s=ARG] [-e=ARG]

Available options:
-a, --addr=ARG The address that you want to sniff. Must be a valid ipv4 addr
ess. Fallsback to 127.0.0.1
-s, --start=ARG The start port for the sniffer. (must be greater than 0)
-e, --end=ARG The end port for the sniffer. (must be less than or equal to 65535)
-h, --help Prints help information
```
