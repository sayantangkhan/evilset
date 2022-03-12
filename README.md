# Evil Set

A variant of Set (and Ultraset) that randomizes attributes across games.

## Things to do

### Core functionality
- [x] Implement dark and light mode and ensure it works with card colors
- [x] Async rendering (using thread for non-wasm)
- [x] Tweak patterns and colors to work well with the color scheme (add brown and blue, and get rid of wavy pattern)
- [ ] Layout buttons in a 3 column grid
- [ ] Implement a timer
- [ ] Enable quitting a game
- [ ] Enable hints
- [ ] Make state persist across runs
- [ ] Async rendering (other methods for wasm)
- [ ] Play sounds
- [ ] Better window controls

### Code cleanup
- [ ] Set up lints for clippy in all 3 crates.
- [ ] Remove extraneous `pub`

### Packaging
- [ ] Set up a WASM web version
- [ ] Package for Flathub
- [ ] Package for MacOS

## Building locally on Linux

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Fedora, if you are using a container to build the application, you will need to install the following packages:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel libXcursor libXrandr libEGL libwayland-egl mesa-dri-drivers`

If you also want to run `rust-analyzer` in the same container, you will need to install additional packages.

`dnf install cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel`

It might also help to install `mold` and use it as a linker to improve iteration speed.

`dnf install clang mold`
