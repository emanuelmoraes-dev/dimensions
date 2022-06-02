use crate::fsdef;

pub fn number(value: u32) -> fsdef![] {
    Box::new(move |_meta, _universe| value)
}

pub fn add(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |meta, universe| f1(meta, universe) + f2(meta, universe))
}

pub fn subtract(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |meta, universe| f1(meta, universe) - f2(meta, universe))
}

pub fn min(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |meta, universe| {
        let p1 = f1(meta, universe);
        let p2 = f2(meta, universe);
        if p1 < p2 {
            p1
        } else {
            p2
        }
    })
}

pub fn max(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |meta, universe| {
        let p1 = f1(meta, universe);
        let p2 = f2(meta, universe);
        if p1 > p2 {
            p1
        } else {
            p2
        }
    })
}
