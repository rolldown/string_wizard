use string_wizard::{Joiner, MagicString};
mod append {
    use super::*;

    #[test]
    fn should_append_content() {
        let mut j = Joiner::default();
        j.append(MagicString::new("*"));
        j.append_raw("123").append_raw("456");
        assert_eq!(j.join(), "*123456");
    }
}