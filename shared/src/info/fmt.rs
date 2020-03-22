use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Bold = 1,
    Italic = 2,
    Underline = 3,
}

impl Ord for FontStyle {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = *self as u8;
        let b = *other as u8;

        a.cmp(&b)
    }
}
