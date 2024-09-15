use wasm_bindgen::prelude::*;

// Expose the `greet` function to JavaScript
#[wasm_bindgen]
pub fn greet() {
    web_sys::console::log_1(&"Hello, Ferris from Rust!".into());
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
