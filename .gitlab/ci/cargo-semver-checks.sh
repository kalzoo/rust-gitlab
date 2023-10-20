#!/bin/sh

set -e

readonly version="0.23.0"
readonly sha256sum="a2167483e3508a22111d828bcbba94e291481de0284332ce47f68b4fcf566fe5"
readonly filename="cargo-semver-checks-x86_64-unknown-linux-musl.tar.gz"

cd .gitlab

echo "$sha256sum  $filename" > cargo-semver-checks.sha256sum
curl -OL "https://github.com/obi1kenobi/cargo-semver-checks/releases/download/v$version/$filename"
sha256sum --check cargo-semver-checks.sha256sum
tar -xf "$filename"
