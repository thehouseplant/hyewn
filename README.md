# hyewn

hyewn - A Hacker News clone written in [Rust](https://www.rust-lang.org/), utilizing [Yew](https://yew.rs/) and [Trunk](https://trunkrs.dev/)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Getting Started

First, you will need to nstall our compile target with the following command:

`rustup target add wasm32-unknown-unknown`

Next, you will need to install Trunk to build and serve the application:

`cargo install trunk wasm-bindgen-cli`

## Local Development

To run the application locally, simply run `trunk serve` and navigate to [http://localhost:8080](http://localhost:8080)

## Release

`trunk build --release`
