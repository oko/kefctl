# `kefctl`

`kefctl` is a control program for KEF wireless speakers, as an alternative to the mobile applications currently available on Android/iOS. Potentially useful for integration with desktop applications like AutoHotKey, Linux users, etc.

`kefdisc` is a companion program to discover KEF speakers on your network via SSDP/UPnP.

## Building

Check out `github.com/oko/libkef` in a sibling directory. Crating and dependency fixes will happen Soon(TM).

Once checked out, then `cargo build` or `cargo run --bin kefctl -- --help`.

## Usage

### Examples

Discover devices on network:

```
192.168.1.100   123456789ABC
[2022-09-24T00:20:41Z INFO  kefdisc] discovered 1 speakers in 1 seconds
```

Set speaker at 192.168.1.100 to use aux input, full volume, left speaker primary, 60 minute standby, and power on:

```
cargo run --bin kefctl -- -i 192.168.1.100 -s aux -v 100 -P left -S s60 -o on
```

### Help output

```
kefctl 0.1.0
Control program for KEF LSX and similar speakers

USAGE:
    kefctl.exe [OPTIONS] --ip <IP>

OPTIONS:
    -g, --get                  Get speaker source settings
    -h, --help                 Print help information
    -i, --ip <IP>              IP address of the KEF speakers to control
    -o, --power <POWER>        Use the left speaker as the primary [possible values: on, off]
    -p, --port <PORT>          TCP port of the KEF speakers to control [default: 50001]
    -P, --primary <PRIMARY>    Use the left speaker as the primary [possible values: right, left]
    -s, --source <SOURCE>      Source input [possible values: wifi, bluetooth, aux, opt, usb]
    -S, --standby <STANDBY>    Standby duration [possible values: s0, s20, s60]
    -v, --volume <VOLUME>      Volume 0-100
    -V, --version              Print version information
```

```
kefdisc 0.1.0

USAGE:
    kefdisc.exe --timeout <TIMEOUT>

OPTIONS:
    -h, --help                 Print help information
    -t, --timeout <TIMEOUT>
    -V, --version              Print version information
```