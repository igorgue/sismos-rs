#!/bin/bash

set -m

npx tailwindcss -i ./src/input.css -o index.css --watch &
trunk serve &

fg %1

killall trunk
