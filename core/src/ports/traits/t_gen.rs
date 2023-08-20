pub trait TImageGen<I, C, F> {
    fn combine(&self, images: Vec<I>) -> I;
    fn color(&self, color: C, width: u32, height: u32) -> I;
    fn text(&self, font: F, bg_color: C, text_color: C, text: &str, width: u32, height: u32) -> I;
}

pub trait TGen<I, C, F, IG: TImageGen<I, C, F>> {
    fn image(&self) -> &IG;
}
