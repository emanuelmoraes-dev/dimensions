use std::rc::Rc;

use image::RgbaImage;
use wasm_bindgen::prelude::*;

use crate::export::x_core::XCore;
use crate::export::x_data::x_image::{XImage, XImageFormatEnum};
use crate::ports::operations::o_map::{get_location, clear_current_map};

#[wasm_bindgen]
pub struct XMap;

#[wasm_bindgen]
impl XMap {
    #[wasm_bindgen(js_name = "getLocation")]
    pub fn get_location(core: &mut XCore, x: i32, y: i32, width: u32, height: u32) -> Option<XImage> {
        let location = get_location(&core.creator, &mut core.universe.current_map, x, y, width, height);
        if let Some(location) = location {
            let image: Rc<RgbaImage> = Rc::clone(&location.image);
            Some(XImage::from_image(XImageFormatEnum::Png, image))
        } else {
            None
        }
    }
    #[wasm_bindgen(js_name = "clearCurrentMap")]
    pub fn clear_current_map(core: &mut XCore) {
        clear_current_map(&mut core.universe);
    }
}
