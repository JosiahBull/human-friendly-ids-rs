repo := 'ghcr.io/josiahbull/send'

# General SQLX files
codegen:
    @export DATABASE_URL=sqlite://$PWD/test.db
    @echo y | @sqlx database drop
    @sqlx database create
    @sqlx migrate run --source ./crates/database/migrations
    @sqlx prepare --workspace

# Run formatting
fmt:
format:
    @cargo +nightly fmt

# Run tests and check for unused dependencies
test:
    @cargo test --all-features --all-targets
    @cargo +nightly udeps
    @cargo +nightly clippy
    @cargo mutants --colors=always --all-features --error true --no-shuffle --iterate -vV
    @cargo deny check
    @cargo semver-checks
    @cargo tree | grep openssl && exit 1 || exit 0

clean:
    @cargo clean
    @git clean -fdX

# Update the insta snapshots
update-snapshots:
    @cargo insta test
    @cargo insta accept

# Publish this crate to crates.io
publish:
    #TODO

default:
    @just --list
