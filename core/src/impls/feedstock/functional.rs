use crate::fsdef;

pub fn number(value: u32) -> fsdef![] {
    Box::new(move |_x, _y, _universe| value)
}

pub fn add(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |x, y, universe| f1(x, y, universe) + f2(x, y, universe))
}

pub fn subtract(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |x, y, universe| f1(x, y, universe) - f2(x, y, universe))
}

pub fn min(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |x, y, universe| {
        let p1 = f1(x, y, universe);
        let p2 = f2(x, y, universe);
        if p1 < p2 {
            p1
        } else {
            p2
        }
    })
}

pub fn max(f1: fsdef![], f2: fsdef![]) -> fsdef![] {
    Box::new(move |x, y, universe| {
        let p1 = f1(x, y, universe);
        let p2 = f2(x, y, universe);
        if p1 > p2 {
            p1
        } else {
            p2
        }
    })
}
