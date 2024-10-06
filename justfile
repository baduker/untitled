default: lint test build

lint:
    cargo fmt -- --check
    cargo clippy --all-targets --all-features -- -D warnings

test:
    echo "Add tests, you fool!"

build:
    cargo build

publish:
    cargo build --release

install:
    cargo install --path .

scrape:
    cargo run -- scrape --url "https://www.kindgirls.com/old/girls.php?id=1633" --full-size-image

