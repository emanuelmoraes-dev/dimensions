pub mod x_image_serializalized;

use wasm_bindgen::prelude::*;

use self::x_image_serializalized::XImageSerialized;
use super::x_image::XImage;

#[wasm_bindgen]
struct XSerialization;

#[wasm_bindgen]
impl XSerialization {
    #[wasm_bindgen]
    pub fn ximage(ximage: &XImage) -> JsValue {
        let image = ximage.image.clone();
        let serialized = XImageSerialized::from(image);
        serde_wasm_bindgen::to_value(&serialized).unwrap()
    }
}
