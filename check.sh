#!/bin/bash
set -eo pipefail

TARGETS=$(find exercises/ -name Cargo.toml | sort)

echo "Check all examples are covered by dependabot"
for target in $TARGETS; do
  DIRNAME=$(dirname "$target")
  grep -Fxq "      - \"/$DIRNAME\"" .github/dependabot.yml || echo "Missing entry in dependabot.yml: $DIRNAME" 1>&2
done
