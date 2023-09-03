pub mod x_image_serializalized;

use wasm_bindgen::prelude::*;

use self::x_image_serializalized::XImageSerialized;
use super::x_image::XImage;

#[wasm_bindgen(js_name = serializeXImage)]
pub fn serialize_ximage(ximage: &XImage) -> JsValue {
    let image = ximage.image.clone();
    let serialized = XImageSerialized::from(image);
    serde_wasm_bindgen::to_value(&serialized).unwrap()
}
