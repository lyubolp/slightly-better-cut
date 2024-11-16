build:
    cargo build

lint:
    cargo clippy

test: build
    cargo test

test_coverage: build
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

push: build lint test
    git push

coverage: test_coverage
    grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov
    grcov . --binary-path ./target/debug/deps/ -s . -t markdown --branch --ignore-not-existing --ignore '../*' --ignore "/*"

run *ARGS: build 
    ./target/debug/sbcut {{ ARGS }}

clean:
    cargo clean
    rm cargo-test-*
    mkdir -p target/coverage

doc:
    cargo doc --no-deps