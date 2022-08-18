use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{decode, encode};

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscale called".into());

    let base64 = decode(encoded_file).unwrap();

    log(&"Image decoded".into());

    let image = image::load_from_memory(&base64).unwrap();

    log(&"loaded image".into());

    let image = image.grayscale();

    log(&"grayscale applied".into());

    let mut buffer = vec![];

    image.write_to(&mut buffer, image::ImageOutputFormat::Png).unwrap();

    log(&"new image written".into());


    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}", encoded_img
    );

    data_url
}
