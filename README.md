# Running with Rust toolchain
1. Install the Rust toolchain
2. run `cargo build --release`
3. run `./target/release/word_chain STARTWORD ENDWORD`

# Running with Docker
1. run `docker build . -t word-chain`
2. run `docker run -it word-chain STARTWORD ENDWORD`
