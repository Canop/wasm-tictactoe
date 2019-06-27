//! A few utilities to deal with the dom

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Document, HtmlElement, console};

pub trait DomusElement {
    fn empty(&self);
}

impl DomusElement for HtmlElement {
    fn empty(&self) {
        while let Some(child) = self.first_element_child() {
            child.remove();
        }
    }
}

pub fn doc() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

// will crash your app when there's no body in the current document.
// This is usually fine.
pub fn body() -> HtmlElement {
    doc().body().unwrap()
}

pub fn js_err<T>(message: &str) -> Result<T, JsValue> {
    Err(JsValue::from_str(message))
}

// TODO in order to eliminate dynamic errors we should use
// an enum of tags instead of taking a str
pub fn tag(tag_name: &str) -> Result<HtmlElement, JsValue> {
    let e: Element = doc().create_element(tag_name)?;
    match e.dyn_into::<HtmlElement>() {
        Ok(e) => Ok(e),
        Err(_) => js_err(&format!("{:?} tag not making a HtmlElement", tag_name)),
    }
}

pub fn tag_class(tag_name: &str, class: &str) -> Result<HtmlElement, JsValue> {
    let e = tag(tag_name)?;
    e.class_list().add_1(class)?;
    Ok(e)
}

pub fn log_str(s: &str) {
    console::log_1(&JsValue::from_str(s));
}

/// log anything. Uses format!
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (log_str(&format!($($arg)*)));
}

