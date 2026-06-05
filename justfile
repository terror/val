set dotenv-load

default:
	just --list

alias f := fmt
alias r := run
alias t := test

all: build test clippy fmt-check

[group: 'dev']
bench:
  cargo bench
  open target/criterion/report/index.html

[group: 'dev']
build:
  cargo build --all --all-targets

[group: 'web']
build-wasm:
  wasm-pack build crates/val-wasm --target web \
    --out-name val \
    --out-dir ../../www/packages/val-wasm

  rm -rf www/packages/val-wasm/.gitignore

[group: 'check']
check:
 cargo check

[group: 'check']
ci: test clippy forbid
  cargo fmt --all -- --check
  cargo update --locked --package val

[group: 'check']
clippy:
  cargo clippy --all --all-targets

[group: 'format']
fmt:
  cargo fmt

[group: 'format']
fmt-check:
  cargo fmt --all -- --check

[group: 'format']
fmt-web:
  cd www && prettier --write .

[group: 'check']
forbid:
  ./bin/forbid

[group: 'misc']
install:
  cargo install -f val

[group: 'dev']
install-dev-deps:
  rustup install nightly
  rustup update nightly
  cargo install cargo-watch
  cargo install wasm-bindgen-cli
  curl -fsSL https://bun.sh/install | bash

[group: 'release']
publish:
  ./bin/publish

[group: 'misc']
readme:
  present --in-place README.md

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
