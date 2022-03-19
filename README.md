# Evil Set

A variant of Set (and Ultraset) that randomizes attributes across games.

## Download or play

Go the the [application web page](https://sayantangkhan.github.io/evilset/) to download native versions or play online.

## Building locally on Linux

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Fedora, if you are using a container to build the application, you will need to install the following packages:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel libXcursor libXrandr libEGL libwayland-egl mesa-dri-drivers`

If you also want to run `rust-analyzer` in the same container, you will need to install additional packages.

`dnf install cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel`

It might also help to install `mold` and use it as a linker to improve iteration speed.

`dnf install clang mold`

## Building locally on macOS

You need to install `rustup`, and possibly XCode. Once that's done, building is just a matter of running `cargo build --release`. Note that this just creates an executable, but not an `.app` bundle.