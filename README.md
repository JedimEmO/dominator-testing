# dominator-testing

Utility crate for writing tests for DOMINATOR UIs.

DOMINATOR is a framework for making high performance web applications using signals.
Head over to https://github.com/Pauan/rust-dominator to learn more!

For more information on testing in the browser, check out the chapter on testing here: https://jedimemo.github.io/dominator-book/techniques_and_patterns/testing.html

## Developing and testing

To run the tests locally, you need a few dependencies on your system.

First of all, you need rust.
Install it following the instructions for your system at https://rustup.rs/

You also need the `wasm32-unknown-unknown` target:

```shell
rustup target add wasm32-unknown-unknown
```

You will also need a web- or chromedriver to be present.
The simplest way is to install chromium, it bundles with its own chromedriver:

```shell
apt install chromium
```

And finally you will need the `wasm-bindgen-cli` tool to be able to run the in-browser tests:

```shell
cargo install wasm-bindgen-cli
```

Now you can run tests with the following commands:

```shell
CHROMEDRIVER=chromium.chromedriver cargo test --target wasm32-unknown-unknown
```