use string_wizard::MagicString;

#[test]
pub fn should_alow_passing_u32_or_usize() {
    let mut s = MagicString::new("x");
    let start_u32 = 0u32;
    let end_u32 = 1u32;
    s.update(start_u32, end_u32, "y");
    assert_eq!(s.to_string(), "y");
    let start_u32 = 0usize;
    let end_u32 = 1usize;
    s.update(start_u32, end_u32, "z");
    assert_eq!(s.to_string(), "z");
}