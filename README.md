# weather-station

Small embeded "Hello World" from [impl Rust for ESP32](https://esp32.implrust.com/e-ink/index.html) book.

Shows current weather in Tokyo with 10 minutes intervals.

# Development

For dev environment setups consult [relevant section of the book](https://esp32.implrust.com/dev-env.html).

In short you'll need:

A few tools

```
cargo install cargo-binstall esp-generate
cargo binstall espflash
```

A toolchain

```
cargo install espup
espup install
```

NB: don't forget to populate your env with

```
. ~/export-esp.sh
```

## WSL2

If you are using WSL2 you will need a few more things set up to pass the USB-Serial to it.

- [usbipd](https://github.com/dorssel/usbipd-win)

```
winget install usbipd
```

- (optional) [WSL USB Manager](https://github.com/nickbeth/wsl-usb-manager) which is a GUI for `usbipd`
