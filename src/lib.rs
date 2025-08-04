use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    text: String,
    target_language: String,
}

#[derive(Serialize)]
pub struct Output {
    translated_text: String,
}

#[no_mangle]
pub extern "C" fn handle(input_ptr: *const u8, input_len: usize) -> *mut u8 {
    let input_data = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let input: Input = serde_json::from_slice(input_data).unwrap();

    let mut dictionary = HashMap::new();
    dictionary.insert(("Hello", "hi"), "नमस्ते");
    dictionary.insert(("Hello", "fr"), "Bonjour");
    dictionary.insert(("Hello", "ur"), "سلام");
    dictionary.insert(("How are you?", "hi"), "आप कैसे हैं?");
    dictionary.insert(("How are you?", "fr"), "Comment ça va?");
    dictionary.insert(("How are you?", "ur"), "آپ کیسے ہیں؟");

    let translation = dictionary
        .get(&(input.text.as_str(), input.target_language.as_str()))
        .unwrap_or(&"Translation not found");

    let output = Output {
        translated_text: translation.to_string(),
    };

    let json_output = serde_json::to_vec(&output).unwrap();
    let boxed_output = json_output.into_boxed_slice();
    let ptr = boxed_output.as_ptr() as *mut u8;
    std::mem::forget(boxed_output);
    ptr
}
