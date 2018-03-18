## Synopsis

A simple web ui for my home VPN client built on BeagleBone Black.

![BBB screenshot](https://raw.githubusercontent.com/olegantonyan/bbb_webui/readme_images/screenshot.png)

![BBB photo](https://raw.githubusercontent.com/olegantonyan/bbb_webui/readme_images/photo.png)

## Installation

Make sure you have rust-nightly and cargo installed.

`cargo run` to run it locally.

To cross-compile it you'll need `arm-unknown-linux-gnueabihf` target.
If using `rustup`:

`rustup target add arm-unknown-linux-gnueabihf`

You'll also need arm gcc toolchain, for example Linaro: https://releases.linaro.org/components/toolchain/binaries/latest/arm-linux-gnueabihf/.

Make sure `arm-linux-gnueabihf-gcc` is in $PATH.

Now you can build it with `cargo build --target=arm-unknown-linux-gnueabihf --release`.

You can also use `make tarball` to build it and pack all required files in one tar.bz2 archive.

To run it on the device via systemd use provided unit file `bbb_webui.service`.

## License

MIT
