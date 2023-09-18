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

mod prepend_append_left_right {
    use super::*;

    #[test]
    fn preserves_intended_order() {
        let mut s = MagicString::new("0123456789");
        s.append_left(5, "A");
        assert_eq!(s.to_string(), "01234A56789");
        s.prepend_right(5, "a");
        s.prepend_right(5, "b");
        s.append_left(5, "B");
        s.append_left(5, "C");
        s.prepend_right(5, "c");

        assert_eq!(s.to_string(), "01234ABCcba56789");
        
        s.prepend_left(5, "<");
        s.prepend_left(5, "{");
        assert_eq!(s.to_string(), "01234{<ABCcba56789");

        s.append_right(5, ">");
        s.append_right(5, "}");
        assert_eq!(s.to_string(), "01234{<ABCcba>}56789");

        s.append_left(5, "(");
        s.append_left(5, "[");
        assert_eq!(s.to_string(), "01234{<ABC([cba>}56789");

        s.prepend_right(5, ")");
        s.prepend_right(5, "]");
        assert_eq!(s.to_string(), "01234{<ABC([])cba>}56789");
    }

    #[test]
    fn preserves_intended_order_at_beginning_of_string() {
        let mut s = MagicString::new("x");
        s.append_left(0, "1");
        s.prepend_left(0, "2");
        s.append_left(0, "3");
        s.prepend_left(0, "4");

        assert_eq!(s.to_string(), "4213x");
    }

    #[test]
    fn preserves_intended_order_at_end_of_string() {
        let mut s = MagicString::new("x");
        s.append_right(1, "1");
        s.prepend_right(1, "2");
        s.append_right(1, "3");
        s.prepend_right(1, "4");

        assert_eq!(s.to_string(), "x4213");
    }


}
mod misc {
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

    #[test]
    fn remove() {
        // should append content
        let mut s = MagicString::new("0123456");
        assert_eq!(s.remove(0, 3).to_string(), "3456");
        assert_eq!(s.remove(3, 7).to_string(), "");
    }
}
