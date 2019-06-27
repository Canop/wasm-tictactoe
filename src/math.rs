use std::ops::Range;
use js_sys::Math::random;

pub fn random_usize(range: Range<usize>) -> usize {
    range.start + (random() * range.end as f64) as usize
}
