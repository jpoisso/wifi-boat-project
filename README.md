# ESP32-S3 Motor Controller

## Dependencies

If you don't have `Python 3.X` installed, you will need it to build certain dependencies.

You will also need this driver for Windows 11 https://www.silabs.com/documents/public/software/CP210x_VCP_Windows.zip 

We'll use `espflash` to push our binaries to the ESP32.


```shell
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-binstall
cargo binstall cargo-espflash 
```

## Setup

We'll use `espup` to setup the proper toolchains for the ESP32.

Run `espup` and follow the prompts.

```shell
espup install
```

## Running on hardware

Connect your `ESP32-S3` device using the `UART` port to your PC.

```shell
cargo espflash flash --monitor
```

## Precomit checks

```shell
cargo clippy --all-targets --all-features --workspace
cargo fmt --all
```