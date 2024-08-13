# ESP32-S3 Wifi Boat Controller

*This is a work in progress.*

## Project Overview

I want to build a fully functionning wifi-controlled 3D printed air boat from scratch.

The boat contains an `ESP32-S3` board which is used to control the boat's motor and rudder using HTTP requests.

You can send `POST` requests to the `/boat` endpoint to update the boat instructions.

```http
### Send boat instruction
POST http://192.168.4.64/boat
Content-Type: application/json

{
  "motor_speed": 255,
  "rudder_angle": 90
}

```

The board runs a web server that can be accessed as long as you're on the same WiFi network. 

I may investigate using AP mode or bluetooth connectivity to make it easier to share wifi network details eventually but at the moment, you have to set `WIFI_SSID` and `WIFI_PASSWORD` environment variables at build time.

## Bill of materials

In Progress.

## Printing the boat

In Progress. 

## Assembling the electronics

In Progress.



## Flashing the firmware

### Build dependencies

You will need:

-  `Rust` version 1.80 or latest
-  `Python` version 3.X with pip and venv (to build esp tooling)

If building on Windows 11, you will probably need to download this driver: https://www.silabs.com/documents/public/software/CP210x_VCP_Windows.zip 

We'll use `espflash` to push our binaries to the ESP32-S3 board.

```shell
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-binstall
cargo binstall cargo-espflash 
```

We'll use `espup` to setup the proper toolchains for the ESP32-S3 board.

```shell
espup install
```

### Flash the binary

Connect your `ESP32-S3` device using the `UART` port to your PC.

```shell
cargo espflash flash --monitor
```

## Contributing

Feel free to submit a pull request!

Run these commands to ensure it will pass Github actions.

```shell
cargo clippy --all-targets --all-features --workspace
cargo fmt --all
```