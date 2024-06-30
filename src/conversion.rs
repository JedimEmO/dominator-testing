use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn as_casted_element<T: JsCast>(ele: &impl JsCast) -> &T {
    ele.dyn_ref::<T>()
        .expect("The provided element is not castable to the requested element type")
}

pub fn as_html_element<T: JsCast>(ele: &T) -> &HtmlElement {
    ele.dyn_ref::<HtmlElement>()
        .expect("The provided element is not castable to HtmlElement")
}