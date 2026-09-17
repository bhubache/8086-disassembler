[private]
default:
  @just --list --unsorted --justfile {{justfile()}} --list-prefix ···· --list-heading ''


fmt:
  cargo fmt --version
  cargo fmt --all -- --check
  cargo clippy --version
  cargo clippy --tests -- -D warnings
