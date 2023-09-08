use string_wizard::MagicString;

use string_wizard::MagicStringOptions;
mod options {
    use super::*;
    #[test]
    fn stores_source_file_information() {
        let s = MagicString::with_options(
            "abc",
            MagicStringOptions {
                filename: Some("foo.js".to_string()),
            },
        );
        assert_eq!(s.filename, Some("foo.js".to_string()))
    }
}

mod append {
    use super::*;
    
    #[test]
    fn should_append_content() {
        // should append content
        let mut s = MagicString::new("abcdefghijkl");
        s.append("xyz");
        assert_eq!(s.to_string(), "abcdefghijklxyz");
        s.append("xyz");
        assert_eq!(s.to_string(), "abcdefghijklxyzxyz");
    }
}

mod prepend {
    use super::*;
    
    #[test]
    fn should_append_content() {
        // should append content
        let mut s = MagicString::new("abcdefghijkl");
        s.prepend("xyz");
        assert_eq!(s.to_string(), "xyzabcdefghijkl");
        s.prepend("xyz");
        assert_eq!(s.to_string(), "xyzxyzabcdefghijkl");
    }
}
