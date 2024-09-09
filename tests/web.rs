//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn sm4_base64_test() {
    let origin = "hello world";
    let encoded_str = sm4_sdk::encrypt_to_base64(&origin);
    let decoded_str = sm4_sdk::decrypt_from_base64(&encoded_str);
    assert_eq!(origin, decoded_str);
}

#[wasm_bindgen_test]
fn sm4_hex_test() {
    let origin = "hello world";
    let encoded_str = sm4_sdk::encrypt_to_hex(&origin);
    let decoded_str = sm4_sdk::decrypt_from_hex(&encoded_str);
    assert_eq!(origin, decoded_str);
}
