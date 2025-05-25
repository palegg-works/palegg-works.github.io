#! /bin/sh
set -e

rm -rf ./docs
dx bundle --platform web --release
cp -rr ./target/dx/palegg-works-github-io/release/web/public ./docs

echo Website source code updated!

