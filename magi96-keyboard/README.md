# Magi96 Keyboard RGB Driver

A userspace driver for controlling RGB lighting on the IQUNIX Magi96 keyboard via USB HID.

## Features

- Control RGB backlight brightness (0-9)
- Set RGB effects (18 different modes)
- Adjust effect speed (0-4)
- Set custom colors using HSV values
- List all available effects

## Installation

### System Dependencies

On Ubuntu/Debian:
```bash
sudo apt-get install libudev-dev libusb-1.0-0-dev
```

### Build

```bash
cargo build --release
```

The binary will be available at `target/release/magi96-keyboard`.

### Permissions Setup (Optional)

To use the driver without sudo, create a udev rule:

```bash
sudo tee /etc/udev/rules.d/99-magi96.rules <<EOF
# IQUNIX Magi96 Keyboard
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="320f", ATTRS{idProduct}=="5088", MODE="0666", TAG+="uaccess"
SUBSYSTEM=="usb", ATTRS{idVendor}=="320f", ATTRS{idProduct}=="5088", MODE="0666", TAG+="uaccess"
EOF
```

Then reload udev rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Unplug and replug the keyboard for the changes to take effect.

## Usage

### List Available Effects

```bash
magi96-keyboard list-effects
```

### Set Brightness

Set brightness level (0-9):
```bash
sudo magi96-keyboard brightness 7
```

### Set RGB Effect

Set an effect by name:
```bash
sudo magi96-keyboard effect wave
sudo magi96-keyboard effect breathe
sudo magi96-keyboard effect vortex
```

### Set Effect Speed

Adjust animation speed (0-4, where 0 is slowest):
```bash
sudo magi96-keyboard speed 2
```

### Set Custom Color

Set color using HSV values (hue, saturation, value - all 0-255):
```bash
sudo magi96-keyboard color 128 255 200
```

Examples:
- Red: `color 0 255 255`
- Green: `color 85 255 255`
- Blue: `color 170 255 255`
- Purple: `color 200 255 255`
- White: `color 0 0 255`

### Get Device Info

```bash
sudo magi96-keyboard info
```

## Available Effects

0. Off
1. Wave
2. Colour Cloud
3. Vortex
4. Mix Colour
5. Breathe
6. Light
7. Slowly Off
8. Stone
9. Laser
10. Starry
11. Flowers Open
12. Traverse
13. Wave Bar
14. Meteor
15. Rain
16. Scan
17. Trigger Colour
18. Center Spread

## Technical Details

- **Vendor ID**: 0x320F
- **Product ID**: 0x5088
- **Protocol**: QMK/VIA HID
- **Interface**: USB 2.4GHz receiver

The driver uses the VIA protocol to communicate with the keyboard's RGB matrix controller over USB HID.

## License

This project is open source.
