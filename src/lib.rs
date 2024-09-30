use wasm_bindgen::prelude::*;

// Expose the `greet` function to JavaScript
#[wasm_bindgen]
pub fn greeting(name: &str) -> JsValue {
    JsValue::from_str("Hello, Ferris from Rust!")
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> Result<i32, JsValue> {
    Ok(a + b)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
