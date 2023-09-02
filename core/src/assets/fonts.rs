#[derive(Clone)]
pub struct Fonts {
    pub roboto_regular: &'static [u8],
    pub roboto_bold: &'static [u8]
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            roboto_regular: include_bytes!("../../assets/fonts/Roboto/Roboto-Regular.ttf"),
            roboto_bold: include_bytes!("../../assets/fonts/Roboto/Roboto-Bold.ttf")
        }
    }
}
