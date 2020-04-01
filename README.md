# Monero Block Explorer

This was mostly built to practice/learn Rust. It may potentially be useful but for now it implements really basic features that can be offered by other explorers. Simplicity can be a good thing though...

## Running

This is quick-start dev server mode.

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repo: `git clone https://github.com/lalanza808/monero-block-explorer && cd monero-block-explorer`
3. Pin the nightly version of rust to the local directory: `rustup override set nightly`
4. [Pick a Monero node](https://moneroworld.com) if you don't have one and apply the `DAEMON_URI` environment variable: `export DAEMON_URI=http://node.supportxmr.com:18081`
5. Run the application: `cargo run`
