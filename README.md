# Evil Set

A variant of Set (and Ultraset) that randomizes attributes across games.

### Things to do
- [ ] Implement floating image buttons that move to a location.
- [ ] Create randomized set cards using the `png` crate.

### Building locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Fedora, if you are using a container to build the application, you will need to install the following packages:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel libXcursor libXrandr libEGL libwayland-egl mesa-dri-drivers`

If you also want to run `rust-analyzer` in the same container, you will need to install additional packages.

`dnf install cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel`

It might also help to install `mold` and use it as a linker to improve iteration speed.

`dnf install clang mold`