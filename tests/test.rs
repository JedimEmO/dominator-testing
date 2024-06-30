#[cfg(test)]
mod test {
    use dominator::{clone, Dom, events, html};
    use dominator_testing::barrier_testing::{Condition, wait_for_query_selector_all_condition};
    use dominator_testing::async_yield;
    use futures_signals::signal::{Mutable, SignalExt};
    use std::time::Duration;
    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::HtmlButtonElement;
    use dominator_testing::conversion::as_casted_element;
    use dominator_testing::element_utilities::get_elements_by_class_name;

    wasm_bindgen_test_configure!(run_in_browser);

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
        dominator_testing::mount_test_dom(rendered);

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
}
