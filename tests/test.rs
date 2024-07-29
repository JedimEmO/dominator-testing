#[cfg(test)]
mod test {
    use dominator::{clone, Dom, events, html, with_node};
    use dominator_testing::barrier_testing::{Condition, wait_for_query_selector_all_condition};
    use dominator_testing::{async_yield, mount_test_dom, with_input_values_map};
    use futures_signals::signal::{Mutable, SignalExt};
    use std::time::Duration;
    use futures_signals::signal_map::MutableBTreeMap;
    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::{console_log, wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::{HtmlButtonElement, HtmlInputElement, InputEvent, InputEventInit};
    use dominator_testing::conversion::as_casted_element;
    use dominator_testing::element_utilities::{get_elements_by_class_name, test_dyn_element_by_id};
    use dominator_testing::prelude::barrier;
    use dominator_testing::signal_testing::with_input_values_map;

    wasm_bindgen_test_configure!(run_in_browser);

    #[ignore]
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
        )
            .await
            .unwrap();
    }

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
            ele.set_value("testing-a");
        });

        test_dyn_element_by_id("b", |ele: &HtmlInputElement| {
            ele.set_value("testing-b");
        });

        barrier(clone!( out => move || {
            console_log!("out: {:?}", out.lock_ref());
            out.lock_ref().get("hi_b") == Some(&"testing-b".to_string())
        }), Duration::from_millis(500), "value in map").await.unwrap();

        barrier(clone!( out => move || {
            out.lock_ref().get("hi_a") == Some(&"testing-a".to_string())
        }), Duration::from_millis(500), "value in map").await.unwrap();
    }
}
