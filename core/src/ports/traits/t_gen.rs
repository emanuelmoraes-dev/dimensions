use image::RgbaImage;

pub trait TImageGen {
    fn combine(&self, images: Vec<RgbaImage>) -> RgbaImage;
}

pub trait TGen<I: TImageGen> {
    fn image(&self) -> &I;
}
