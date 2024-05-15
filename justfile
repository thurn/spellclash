set positional-arguments

code-review: check-format build clippy test check-docs

check:
    cargo check --manifest-path src-tauri/Cargo.toml --workspace --all-targets --all-features

check-warnings:
    RUSTFLAGS="--deny warnings" cargo check --manifest-path src-tauri/Cargo.toml --workspace --all-targets --all-features --exclude "protos"

build:
    cargo build --manifest-path src-tauri/Cargo.toml --all-targets --all-features

run *args='':
    npm run tauri dev

test:
    cargo test --manifest-path src-tauri/Cargo.toml

doc:
    cargo doc --manifest-path src-tauri/Cargo.toml

logs:
  tail -f "$HOME/Library/Application Support/com.spellclash.spellclash/spellclash.log"

clippy:
    cargo clippy --manifest-path src-tauri/Cargo.toml --workspace -- -D warnings -D clippy::all

# Reformats code. Requires nightly because several useful options (e.g. imports_granularity) are
# nightly-only
fmt:
    # Manifest path seems to not work?
    cd src-tauri && cargo +nightly fmt

nim *args='':
    cargo run --manifest-path src-tauri/Cargo.toml --bin nim -- "$@"

run_matchup *args='':
    cargo run --manifest-path src-tauri/Cargo.toml --bin run_matchup -- "$@"

run_tournament *args='':
    cargo run --manifest-path src-tauri/Cargo.toml --bin run_tournament -- "$@"

check-format:
    # Manifest path seems to not work?
    cd src-tauri && cargo +nightly fmt -- --check

check-docs:
    RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links -D rustdoc::private-intra-doc-links -D rustdoc::bare-urls" cargo doc --manifest-path src-tauri/Cargo.toml --all

outdated:
    # Check for outdated dependencies, consider installing cargo-edit and running 'cargo upgrade' if this fails
    cargo outdated ---manifest-path src-tauri/Cargo.toml -exit-code 1

clear-data:
    rm -r ~/Library/Application\ Support/com.spellclash.spellclash/
    mkdir ~/Library/Application\ Support/com.spellclash.spellclash/

upgrade:
    cargo upgrade --manifest-path src-tauri/Cargo.toml --workspace

machete:
    cargo machete --manifest-path src-tauri/Cargo.toml --fix

remove-unused-deps: machete

@dropbox:
    find . -name '*conflicted*' -delete
    xattr -w com.dropbox.ignored 1 src-tauri/target
    xattr -w com.dropbox.ignored 1 node_modules

internal_clean:
  rm -rf src-tauri/target/debug
  rm -rf src-tauri/target/release
  rm -rf src-tauri/target/tmp

clean: internal_clean dropbox
