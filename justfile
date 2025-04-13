set dotenv-load

default:
	just --list

alias f := fmt
alias r := run
alias t := test

all: build test clippy fmt-check

[group: 'dev']
build:
  cargo build

[group: 'web']
build-wasm:
  wasm-pack build crates/val-wasm --target web \
    --out-name val \
    --out-dir ../../www/packages/val-wasm

[group: 'check']
check:
 cargo check

[group: 'check']
ci: test clippy forbid
  cargo +nightly fmt --all -- --check
  cargo update --locked --package null

[group: 'check']
clippy:
  cargo clippy --all --all-targets

[group: 'format']
fmt:
  cargo +nightly fmt

[group: 'format']
fmt-check:
  cargo +nightly fmt --all -- --check

[group: 'format']
fmt-web:
  cd www && prettier --write .

[group: 'check']
forbid:
  ./bin/forbid

[group: 'misc']
install:
  cargo install -f null

[group: 'dev']
install-dev-deps:
  rustup install nightly
  rustup update nightly
  cargo install cargo-watch
  cargo install wasm-bindgen-cli
  curl -fsSL https://bun.sh/install | bash

[group: 'release']
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  gh repo clone https://github.com/terror/null tmp/release
  cd tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release

[group: 'dev']
run *args:
  cargo run {{ args }}

[group: 'web']
serve-web: build-wasm
  cd www && bun run dev

[group: 'test']
test:
  cargo test

[group: 'test']
test-release-workflow:
  -git tag -d test-release
  -git push origin :test-release
  git tag test-release
  git push origin test-release

[group: 'release']
update-changelog:
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
