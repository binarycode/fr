default:
    @just --list --unsorted

fmt:
    treefmt --ci

clean:
    cargo clean

check:
    cargo check --all-targets
    cargo clippy
    cargo deny check
    cargo audit
    cargo outdated

run *ARGS:
    cargo run {{ ARGS }}

test *ARGS:
    cargo test {{ ARGS }}
