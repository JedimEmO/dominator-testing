# dominator-testing

Utility crate for writing tests for DOMINATOR UIs.

DOMINATOR is a framework for making high performance web applications using signals.
Head over to https://github.com/Pauan/rust-dominator to learn more!

For more information on testing in the browser, check out the chapter on testing here: https://jedimemo.github.io/dominator-book/techniques_and_patterns/testing.html

## Examples

To illustrate how to test with this crate, here's an example taken from the dogfooded unit tests.

### Basic test

This tests creates a small component with a button that adds 100 rows when clicked.
The test then verifies that 100 rows are in fact spawned when clicking the button, by using the `wait_for_query_selector_all_condition` function.

```rust
#[wasm_bindgen_test]
async fn basic_dom_test() {
    let count = Mutable::new(0);

    let rendered: Dom = html!("div", {
        .child(html!("button", {
            .class("clickme")
            .text("click me!")
            .event(clone!(count => move |_event: events::Click| {
                spawn_local(clone!(count => async move {
                    for _ in 0..100 {
                        count.set(count.get() + 1);
                        async_yield().await;
                    }
                }));
            }))
        }))
        .children_signal_vec(count.signal().map(|v| {
            (0..v).map(|v| html!("div", {
                .class("child-row")
                .text(&v.to_string())
            })).collect::<Vec<_>>()
        }).to_signal_vec())
    });

    // attach the dominator dom to the real dom to make things work for this test
    mount_test_dom(rendered);

    // Retrieve a typed handle to the clickme button
    let ele = get_elements_by_class_name("clickme");
    let element = ele.first().unwrap();
    let btn_ele = as_casted_element::<HtmlButtonElement>(element);

    btn_ele.click();

    // Wait for the query to yield at least 100 nodes
    wait_for_query_selector_all_condition(
        ".child-row",
        Condition::Fn(Box::new(|node_list| node_list.length() >= 100)),
        Duration::from_millis(500),
    ).await.unwrap();
}
```

### Monitoring input values in an element

Another interesting thing to do, is to set up monitoring of the input values of all "input" dom elements within a dominator UI:

```rust
#[wasm_bindgen_test]
async fn test_with_input_map() {
    let mut out: MutableBTreeMap<String, String> = Default::default();

    fn cmp_under_test() -> Dom {
        html!("div", {
            .child(html!("input", {
                .attr("name", "hi_a")
                .attr("id", "a")
            }))
            .child(html!("div", {
                .child(html!("input", {
                    .attr("name", "hi_b")
                    .attr("id", "b")
                }))
            }))
        })
    }

    let dom = html!("div", {
        .with_input_values_map!(out)
        .child(cmp_under_test())
    });

    mount_test_dom(dom);

    barrier(clone!( out => move || {
        out.lock_ref().keys().len() == 2
    }), Duration::from_millis(500), "number of inputs in map").await.unwrap();

    test_dyn_element_by_id("a", |ele: &HtmlInputElement| {
        let mut init = InputEventInit::new();
        init.data(Some("testing-a"));
        let event = InputEvent::new_with_event_init_dict("input", &init).unwrap();
        ele.dispatch_event(event.as_ref()).unwrap();
    });

    test_dyn_element_by_id("b", |ele: &HtmlInputElement| {
        let mut init = InputEventInit::new();
        init.data(Some("testing-b"));
        let event = InputEvent::new_with_event_init_dict("input", &init).unwrap();
        ele.dispatch_event(event.as_ref()).unwrap();
    });

    barrier(clone!( out => move || {
        out.lock_ref().get("hi_b") == Some(&"testing-b".to_string())
    }), Duration::from_millis(200), "value in map").await.unwrap();

    barrier(clone!( out => move || {
        out.lock_ref().get("hi_a") == Some(&"testing-a".to_string())
    }), Duration::from_millis(200), "value in map").await.unwrap();
}
```

The important part here is the wrapper made here:

```rust
let dom = html!("div", {
    .with_input_values_map!(out)
    .child(cmp_under_test())
});
```

The `with_input_values_map!` macro will attach a future to the wrapper, which monitors all inputs in the tree using `query_selector_all`, and attaches event handles to it so that the out map holds name,value entries for all inputs.

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