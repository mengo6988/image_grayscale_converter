// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::general_purpose};
// use base64::decode;
// deprecated
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
	log(&"Grayscale called".into());

	// let base64_to_vector = engine::GeneralPurpose::new(&alphabet::URL_SAFE,general_purpose::NO_PAD).decode(encoded_file).unwrap();
	let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
	// let base64_to_vector = decode(encoded_file).unwrap();
	log(&"image decoded".into());

	let mut img = load_from_memory(&base64_to_vector).unwrap();

	log(&"image loaded".into());

	img = img.grayscale();
	log(&"grayscale effect applied".into());

	let mut buffer = vec![];
	img.write_to(&mut buffer, Png).unwrap();
	log(&"new image written".into());

	let encoded_img = general_purpose::STANDARD.encode(&buffer);
	let data_url: String = format!(
		"data:image/png;base64,{}",
		encoded_img
	);

	data_url
}
