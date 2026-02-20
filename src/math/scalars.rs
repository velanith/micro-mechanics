pub fn rem_euclid_f32(x: f32, rhs: f32) -> f32 {
    let r = libm::fmodf(x, rhs);
    if r < 0.0 { r + rhs } else { r }
}
