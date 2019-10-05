//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
#[macro_use]
extern crate serde_json;
use dessert_filesize_core::filesize;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn no_options() {
    assert_eq!(
        filesize(&JsValue::from_f64(500.0), &JsValue::UNDEFINED),
        "500 B"
    );
}

fn test(val: f64, res: &str, options: &serde_json::Value) {
    assert_eq!(
        filesize(
            &JsValue::from_f64(val),
            &JsValue::from_serde(&options).unwrap()
        ),
        res
    );
}

#[wasm_bindgen_test]
fn spacer() {
    let options = json!({
        "spacer": "   ",
    });
    test(500.0, "500   B", &options);
}

#[wasm_bindgen_test]
fn bits() {
    let options = json!({
        "bits": true,
    });
    test(500.0, "3.91 Kb", &options);
}

#[wasm_bindgen_test]
fn exponent() {
    let options = json!({
        "exponent": 2,
    });
    test(500_000_000.0, "476.84 MB", &options);
}
