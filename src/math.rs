use js_sys::Math::random;

pub fn random_usize(max: usize) -> usize {
    (random() * max as f64) as usize
}
