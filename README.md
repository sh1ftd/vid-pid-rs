# vid-pid-rs

A simple command-line utility to list USB HID devices with their vendor IDs, product IDs, and other details.

## Features

- Lists all connected USB HID devices
- Displays detailed device information:
  - VID/PID values
  - Serial numbers
  - Manufacturer and product strings
  - Release numbers and interface information
  - Bus type and usage information

## Installation

### Prerequisites

- Rust and Cargo

### Compile from source

```
git clone https://github.com/sh1ftd/vid-pid-rs.git
cd vid-pid-rs
cargo build --release
```
