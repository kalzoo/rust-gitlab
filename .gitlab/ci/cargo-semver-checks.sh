#!/bin/sh

set -e

readonly version="0.30.0"
readonly sha256sum="cf85eb70bd07db1650a4d2938db4aeac4273c30f255bf42797353515d7643414"
readonly filename="cargo-semver-checks-x86_64-unknown-linux-musl.tar.gz"

cd .gitlab

echo "$sha256sum  $filename" > cargo-semver-checks.sha256sum
curl -OL "https://github.com/obi1kenobi/cargo-semver-checks/releases/download/v$version/$filename"
sha256sum --check cargo-semver-checks.sha256sum
tar -xf "$filename"
