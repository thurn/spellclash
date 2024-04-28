set positional-arguments

code-review: check-format build clippy test check-docs

check:
    cargo check --workspace --all-targets --all-features

check-warnings:
    RUSTFLAGS="--deny warnings" cargo check --workspace --all-targets --all-features --exclude "protos"

build:
    cargo build --all-targets --all-features

run *args='':
    cargo run -- $@

test:
    cargo test

doc:
    cargo doc

logs:
  tail -f "$HOME/Library/Application Support/com.spellclash.spellclash/spellclash.log"

clippy:
    cargo clippy --workspace -- -D warnings -D clippy::all

# Reformats code. Requires nightly because several useful options (e.g. imports_granularity) are
# nightly-only
fmt:
    cargo +nightly fmt

nim *args='':
    cargo run --bin nim -- "$@"

run_matchup *args='':
    cargo run --bin run_matchup -- "$@"

run_tournament *args='':
    cargo run --bin run_tournament -- "$@"

check-format:
    cargo +nightly fmt -- --check

check-docs:
    RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links -D rustdoc::private-intra-doc-links -D rustdoc::bare-urls" cargo doc --all

outdated:
    # Check for outdated dependencies, consider installing cargo-edit and running 'cargo upgrade' if this fails
    cargo outdated --exit-code 1

upgrade:
    cargo upgrade --workspace

machete:
    cargo machete --fix

remove-unused-deps: machete

@dropbox:
    find . -name '*conflicted*' -delete
    mkdir -p target
    xattr -w com.dropbox.ignored 1 target/

internal_clean:
  rm -r target/debug
  rm -r target/release
  rm -r target/tmp

clean: internal_clean dropbox
