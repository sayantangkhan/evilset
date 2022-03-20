## About

Evilset is a variant of [Set](https://en.wikipedia.org/wiki/Set_(card_game)) that randomizes visual attributes across games: this reduces the reliance on muscle memory and levels the playing field between experienced players and novices.

## Play Evilset
Evilset can be played in the browser as well in a native application for Linux, macOS, and Windows.
- [Web version](https://sayantangkhan.github.io/evilset/web/index.html): Tested on Firefox on Linux and Safari on iOS, but should work on most modern web browsers.
- [Linux version](https://sayantangkhan.github.io/evilset/linux/evilset_linux): This works on Wayland, and on X11, it starts up successfully with probability 1/n, where n is the number of hyperthreads on your computer, due to a bug in X11.
- [macOS version](https://sayantangkhan.github.io/evilset/macos/Evilset.app.zip): Gatekeeper's default settings prevent the downloaded app from running. To work around this, either disable Gatekeeper, or download the source code, and run `cargo bundle --release` to create an `.app` bundle that will run on your Mac.
- Build from source: See the [Github repository](https://github.com/sayantangkhan/evilset) for instructions on how to build it from source.
