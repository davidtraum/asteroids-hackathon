#[test]
fn rudimentary() {
    assert_eq!(to_rad(0.), 0.);
    assert_eq!(to_rad(180.), std::f32::consts::PI);
    assert_eq!(to_rad(360.), std::f32::consts::PI * 2.);
}
