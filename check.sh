#!/bin/bash
set -eo pipefail

mkdir -p target

find exercises/ -name Cargo.toml | sort > target/on-disk.txt
json5 .zed/settings.json | jq --raw-output '.lsp."rust-analyzer".initialization_options.linkedProjects[]' > target/zed.txt
json5 .vscode/settings.json | jq --raw-output '."rust-analyzer.linkedProjects"[]' > target/vscode.txt
cat .github/dependabot.yml | yq --raw-output '.updates[0].directories[] + "/Cargo.toml" | sub("^/";"")' > target/dependabot.txt

diff --unified=0 --from-file target/on-disk.txt target/dependabot.txt target/vscode.txt target/zed.txt && rm -r target || (echo "Projects and configs do not match"; exit 1)
