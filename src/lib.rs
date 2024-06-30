pub mod barrier_testing;
pub mod conversion;
pub mod element_utilities;

use dominator::{body, Dom};
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub mod prelude {
    pub use super::mount_test_dom;
    pub use super::async_yield;
    pub use super::barrier_testing::*;
    pub use super::element_utilities::*;
    pub use super::conversion::*;
}

/// Attaches the provided dom node to the browser body node, replacing the previous children.
/// Replacing is important to avoid bleeding between tests
pub fn mount_test_dom(dom: Dom) {
    dominator::replace_dom(
        &body(),
        &body().first_child().unwrap(),
        dom,
    );
}

/// Utility for yielding time back to the browser.
/// This is necessary because of the single threaded nature of the browser environment
pub async fn async_yield() {
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null()))
        .await
        .unwrap();
}

#[derive(Error, Debug)]
pub enum DominatorTestingError {
    #[error("Timeout error waiting for {0}")]
    BarrierTimeOut(String),
}

