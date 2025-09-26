#!/bin/bash

set -eo pipefail

echo "TODO: use $1 as base"

rm -rf public/
mkdir -p public/

# build mdbook
pushd book
  mdbook build
popd
mv book/target/* ./public/

# build slides
pushd slides
  npm ci
  for f in *.md; do
    npm run build -- --out "dist/${f%.md}" --base "/slides/${f%.md}/" "$f"
  done
popd
mv slides/dist ./public/slides
