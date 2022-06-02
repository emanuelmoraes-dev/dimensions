#[derive(Clone, Copy)]
pub enum LocationType {
    Ground,
    Cave,
    Wall {
        quantity: u32
    },
    Gram {
        quantity: u32,
        min_xratio: f32,
        max_xratio: f32
    },
    River {
        quantity: u32,
        min_xratio: f32,
        max_xratio: f32
    },
}
