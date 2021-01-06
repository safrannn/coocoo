use super::image_library::IMAGE_LIBRARY;
use super::*;
use wasm_bindgen::prelude::*;

log_rule!();

#[wasm_bindgen]
pub fn darken(image_id: i32, value: i32) -> i32 {
    //image_id here not synced
    let mut result_image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in 0..result_image_data.pixels.len() {
        if result_image_data.pixels[i] > value as u8 {
            result_image_data.pixels[i] -= value as u8;
        } else {
            result_image_data.pixels[i] = 0;
        }
    }
    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        result_image_data.width,
        result_image_data.height,
        result_image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn blank_image(width: i32, height: i32) {
    let pixel_total: usize = (width * height * 3) as usize;
    let mut result_image_data: Vec<u8> = vec![0; pixel_total];
    for i in 0..pixel_total {
        result_image_data[i] = 100;
    }

    IMAGE_LIBRARY
        .lock()
        .unwrap()
        .add_image("".to_string(), width, height, result_image_data);
}
