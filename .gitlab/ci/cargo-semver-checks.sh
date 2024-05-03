#!/bin/sh

set -e

readonly version="0.31.0"
readonly sha256sum="4b40df7c8877451b3c31d33399f54bc1d988cad3a2712764ce632a7425cc57df"
readonly filename="cargo-semver-checks-x86_64-unknown-linux-musl.tar.gz"

cd .gitlab

echo "$sha256sum  $filename" > cargo-semver-checks.sha256sum
curl -OL "https://github.com/obi1kenobi/cargo-semver-checks/releases/download/v$version/$filename"
sha256sum --check cargo-semver-checks.sha256sum
tar -xf "$filename"
