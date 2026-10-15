run CMD: build
   ./sss {{CMD}}

build:
   cargo build --release
   cp ./target/release/sss .
