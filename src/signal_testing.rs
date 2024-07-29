use std::future::Future;
use std::time::Duration;
use dominator::clone;
use futures_signals::signal_map::MutableBTreeMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::console_log;
use web_sys::{Element, HtmlInputElement, InputEvent};
use crate::async_yield;
use crate::prelude::as_casted_element;

#[macro_export]
macro_rules! with_input_values_map {
    ($this:ident, $map:tt) => {{
        dominator::apply_methods!($this, {
            .with_node!(element => {
                .apply(|builder| {
                    builder.future(with_input_values_map(element.clone().into(), $map.clone()))
                })
            })
        })
    }};
}

pub fn with_input_values_map(element: Element, out: MutableBTreeMap<String, String>) -> impl Future<Output=()> + 'static {
    async move {
        let inputs = element.query_selector_all("input").unwrap();

        loop {
            for idx in 0..inputs.length() {
                let item = inputs.item(idx).unwrap();
                let html_input = as_casted_element::<HtmlInputElement>(&item);

                let name = html_input.name();
                let value = html_input.value();

                let mut lock = out.lock_mut();
                lock.insert_cloned(name, value);
            }

            gloo_timers::future::sleep(Duration::from_millis(1)).await;
        }
    }
}