#!/bin/sh

set -e

readonly version="0.9.53"
readonly sha256sum="b07def6a5e5521481eb5853e5f17650be406ce8c57ce917a90d2866c788e5967"
readonly filename="x86_64-unknown-linux-gnu.tar.gz"

cd .gitlab

echo "$sha256sum  $filename" > cargo-nextest.sha256sum
curl -OL "https://get.nexte.st/$version/$filename"
sha256sum --check cargo-nextest.sha256sum
tar -xf "$filename"
