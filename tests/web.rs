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
fn string_as_param() {
    assert_eq!(
        filesize(&JsValue::from_str("500"), &JsValue::UNDEFINED),
        "500 B"
    );
}

#[wasm_bindgen_test]
fn base() {
    let options = json!({
        "base": 10,
    });
    test(5000.0, "5 kB", &options);
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

#[wasm_bindgen_test]
fn fullform() {
    let options = json!({
        "fullform": true,
    });
    test(500.0, "500 bytes", &options);
}

#[wasm_bindgen_test]
fn fullforms() {
    let options = json!({
        "fullform": true,
        "fullforms": ["oui"],
    });
    test(12.0, "12 oui", &options);
}

#[wasm_bindgen_test]
fn locale() {
    let options = json!({
        "locale": "de",
    });
    test(500_000.0, "488,28 KB", &options);
}

#[wasm_bindgen_test]
fn round() {
    let options = json!({
        "round": 5,
    });
    test(50000.0, "48.82813 KB", &options);
}

#[wasm_bindgen_test]
fn separator() {
    let options = json!({
        "separator": "oui",
    });
    test(50000.0, "48oui83 KB", &options);
}

#[wasm_bindgen_test]
fn standard() {
    let options = json!({
        "standard": "iec",
    });
    test(5000.0, "4.88 KiB", &options);
}

#[wasm_bindgen_test]
fn unix() {
    let options = json!({
        "unix": true,
    });
    test(500.0, "500", &options);
}
