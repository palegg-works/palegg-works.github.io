#! /bin/sh
set -e

dx bundle --platform web --release
cp -r ./target/dx/palegg-works-github-io/release/web/public .

echo Website source code updated!

