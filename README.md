# `kefctl`

`kefctl` is a control program for KEF wireless speakers, as an alternative to the mobile applications currently available on Android/iOS. Potentially useful for integration with desktop applications like AutoHotKey, Linux users, etc.

## Building

Check out `github.com/oko/libkef` in a sibling directory. Crating and dependency fixes will happen Soon(TM).

Once checked out, then `cargo build` or `cargo run -- --help`.

```
Control program for KEF LSX and similar speakers

USAGE:
    kefctl.exe [OPTIONS] --ip <IP>

OPTIONS:
    -h, --help               Print help information
    -i, --ip <IP>            IP address of the KEF speakers to control
    -p, --port <PORT>        TCP port of the KEF speakers to control [default: 50001]
    -s, --source <SOURCE>    Source input [possible values: wifi, bluetooth, aux, opt, usb]
    -v, --volume <VOLUME>    Volume 0-100
    -V, --version            Print version information
    -x, --off                Turn off the speakers (only command sent if present)
```