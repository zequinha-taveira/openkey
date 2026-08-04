#!/usr/bin/env sh
# Stage the OpenKey documentation as a static site for the Freebuff
# hosting build (output must land in dist/ and the command must exit).
set -e

rm -rf dist
mkdir -p dist

# Documentation index + key top-level docs
cp -r docs README.md spec.md Product.md Ecosystem.md dist/

echo "Staged documentation into dist/"
