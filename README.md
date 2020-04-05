# Monero Block Explorer

This simple web application allows users to search, visualize (soon), and interact with the [Monero](https://getmonero.org) blockchain.

## Running

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repo: `git clone https://github.com/lalanza808/monero-block-explorer && cd monero-block-explorer`
3. Pin the nightly version of rust to the local directory: `rustup override set nightly`
4. [Pick a Monero node](https://moneroworld.com) if you don't have one and apply the `DAEMON_URI` environment variable: `export DAEMON_URI=http://node.supportxmr.com:18081`
5. Run the application: `cargo run`
