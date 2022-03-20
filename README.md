# Evil Set

A variant of Set (and Ultraset) that randomizes attributes across games.

## Download or play

Go the the [application web page](https://sayantangkhan.github.io/evilset/) to download native versions or play online.

## Building locally on Linux

### Fedora 34 or newer

Make sure you are using the latest version of stable rust by running `rustup update`. You will also need to install the following packages:

`sudo dnf install clang clang-devel clang-tools-extra mold speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel libXcursor libXrandr libEGL libwayland-egl mesa-dri-drivers`

If you also want to run `rust-analyzer` in the same container, you will need to install additional packages.

`dnf install cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel`

### Ubuntu 18.04 or newer

Make sure you are using the latest version of stable rust by running `rustup update`. You will also need to install the following packages:
`sudo apt install clang build-essential git libxcb1-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev`

## Building locally on macOS

You need to install `rustup`, and possibly XCode. Once that's done, building is just a matter of running `cargo build --release`. Note that this just creates an executable, but not an `.app` bundle.
