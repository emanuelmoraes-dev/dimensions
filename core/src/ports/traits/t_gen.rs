use crate::ports::models::style::{TextStyle, AlignEnum};

pub trait TImageGen<I> {
    fn combine(&self, bg_color: &[u8; 4], align_x: &AlignEnum, align_y: &AlignEnum, images: Vec<&I>) -> I;
    fn color(&self, color: &[u8; 4], width: u32, height: u32) -> I;
    fn text(&self, color: &[u8; 4], width: u32, height: u32, text_style: TextStyle, text: &str) -> I;
}

pub trait TGen<I, IG: TImageGen<I>> {
    fn image(&self) -> &IG;
}
