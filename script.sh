
sudo apt install build-essential git cmake clang
sudo apt install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev
sudo apt -y install libssl-dev

# install rust 
# install from https://www.rust-lang.org/tools/install
rustup component list --installed
rustup show

rustup default nightly
rustup update

cargo install xargo

rustup component add rust-src

xargo build

cargo build --target wasm32-unknown-unknown
cargo fix --lib -p wasm_c_lfu
cargo fix --lib -p wasm_pair_lfu

rustup target add wasm32-unknown-unknown --toolchain nightly

cargo run --bin benchmark --release
cargo run --bin benchmark --debug
