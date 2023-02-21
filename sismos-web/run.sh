#!/bin/bash

set -m

cd ../
cargo run --bin sismos-api &
cd sismos-web || return
npx tailwindcss -i ./src/input.css -o index.css --watch &
trunk serve &

fg %1

killall trunk
