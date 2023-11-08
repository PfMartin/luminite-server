# luminite-server

## Description

Uses an ESP32-C3 with Risc-V to create a webserver, that allows controlling an LED strip.

## Onboard LED Colors

The Onboard LED signals the state of the webserver

- Green: The webserver is ready and waiting for connection
- Red: The ESP32 tries to connect to the WIFI and is not done setting up starting the webserver

## API

The webserver has 2 endpoints:

- `/`: Default landing page with a welcome message. Sets the color of the LED strip to the default color: `255, 150, 50`
- `/set-color`: Can be used to set the color of all leds on the led strip. Examples
  - `http://luminite-server/set-color?r=255&g=255&b=255`
  - `http://luminite-server/set-color?r=0&g=255&b=0`

## Development Setup

- The development environment can be setup by following this tutorial [The Rust on ESP Book](https://esp-rs.github.io/book/overview/index.html)
- Choose the `std` approach
- Get your ESP32-C3 board
- Get a WS2812B LED Strip

## Configuration

- You can configure the server via a `cfg.toml`
- Just use the `cfg.toml.example` as a template