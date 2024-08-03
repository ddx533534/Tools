#![no_main]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
// 使用 wasm-pack 打包为wasm

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("hello, {}!", name));
}

#[wasm_bindgen]
pub fn add_two(a1: i32, a2: i32) -> i32 {
    a1 + a2
}

