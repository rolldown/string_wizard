use std::borrow::Cow;

use string_wizard::MagicString;
use string_wizard::MagicStringOptions;
use string_wizard::UpdateOptions;

trait MagicStringExt<'text> {
    fn overwrite(
        &mut self,
        start: usize,
        end: usize,
        content: impl Into<Cow<'text, str>>,
    ) -> &mut Self;
}

impl<'text> MagicStringExt<'text> for MagicString<'text> {
    fn overwrite(
        &mut self,
        start: usize,
        end: usize,
        content: impl Into<Cow<'text, str>>,
    ) -> &mut Self {
        self.update_with(
            start,
            end,
            content,
            UpdateOptions {
                overwrite: true,
                ..Default::default()
            },
        )
    }
}

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

mod clone {
    use super::*;

    #[test]
    fn should_clone_a_magic_string() {
        let mut s = MagicString::new("abcdefghijkl");
        s.overwrite(3, 9, "XYZ");
        let c = s.clone();

        assert_eq!(c.to_string(), "abcXYZjkl")
    }
}

mod overwrite {
    use super::*;

    #[test]
    fn should_replace_characters() {
        let mut s = MagicString::new("abcdefghijkl");
        s.overwrite(5, 8, "FGH");
        assert_eq!(s.to_string(), "abcdeFGHijkl");
    }

    // #[test]
    // fn should_throw_an_error_if_overlapping_replacements_are_attempted() {
    //     let mut s = MagicString::new("abcdefghijkl");
    //     s.overwrite(7, 11, "xx");
    //     assert!(std::panic::catch_unwind(|| {
    //         s.clone().overwrite(8, 12, "yy");
    //     })
    //     .is_err())
    // }
}

mod relocate {
    use super::*;

    #[test]
    fn moves_content_from_the_start() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(0, 3, 6);
        assert_eq!(s.to_string(), "defabcghijkl");
    }

    #[test]
    fn moves_content_to_the_start() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(3, 6, 0);
        assert_eq!(s.to_string(), "defabcghijkl");
    }

    #[test]
    fn moves_content_from_the_end() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(9, 12, 6);
        assert_eq!(s.to_string(), "abcdefjklghi");
    }

    #[test]
    fn moves_content_to_the_end() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(6, 9, 12);
        assert_eq!(s.to_string(), "abcdefjklghi");
    }

    #[test]
    fn ignores_redundant_move() {
        let mut s = MagicString::new("abcdefghijkl");
        s.prepend_right(9, "X")
            .relocate(9, 12, 6)
            .append_left(12, "Y")
            // this is redundant – [6,9] is already after [9,12]
            .relocate(6, 9, 12);

        assert_eq!(s.to_string(), "abcdefXjklYghi");
    }

    #[test]
    fn moves_content_to_the_middle() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(3, 6, 9);
        assert_eq!(s.to_string(), "abcghidefjkl");
    }

    #[test]
    fn handles_multiple_moves_of_the_same_snippet() {
        let mut s = MagicString::new("abcdefghijkl");

        s.relocate(0, 3, 6);
        assert_eq!(s.to_string(), "defabcghijkl");

        s.relocate(0, 3, 9);
        assert_eq!(s.to_string(), "defghiabcjkl");
    }

    #[test]
    fn handles_moves_of_adjacent_snippets() {
        let mut s = MagicString::new("abcdefghijkl");

        s.relocate(0, 2, 6);
        assert_eq!(s.to_string(), "cdefabghijkl");
        s.relocate(2, 4, 6);
        assert_eq!(s.to_string(), "efabcdghijkl");
    }

    #[test]
    fn handles_moves_to_same_index() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(0, 2, 6).relocate(3, 5, 6);
        assert_eq!(s.to_string(), "cfabdeghijkl");
    }

    #[test]
    #[should_panic]
    fn refuses_to_move_a_selection_to_inside_itself() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(3, 6, 3);
    }
    #[test]
    #[should_panic]
    fn refuses_to_move_a_selection_to_inside_itself2() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(3, 6, 4);
    }
    #[test]
    #[should_panic]
    fn refuses_to_move_a_selection_to_inside_itself3() {
        let mut s = MagicString::new("abcdefghijkl");
        s.relocate(3, 6, 6);
    }

    #[test]
    fn allows_edits_of_moved_content() {
        let mut s1 = MagicString::new("abcdefghijkl");
        s1.relocate(3, 6, 9);
        s1.overwrite(3, 6, "DEF");
        assert_eq!(s1.to_string(), "abcghiDEFjkl");
        let mut s2 = MagicString::new("abcdefghijkl");
        s2.relocate(3, 6, 9);
        s2.overwrite(4, 5, "E");
        assert_eq!(s2.to_string(), "abcghidEfjkl");
    }

    #[test]
    fn moves_content_inserted_at_end_of_range() {
        let mut s = MagicString::new("abcdefghijkl");
        s.append_left(6, "X").relocate(3, 6, 9);
        assert_eq!(s.to_string(), "abcghidefXjkl");
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
