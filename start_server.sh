#!/bin/bash
set -eu

# Starts a local web-server that serves the contents of the `doc/` folder,
# which is the folder to where the web version is compiled.

cargo install basic-http-server

echo "open http://127.0.0.1:8080"

(cd web && basic-http-server --addr 0.0.0.0:8080 .)
