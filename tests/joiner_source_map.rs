use string_wizard::{Joiner, JoinerOptions, MagicString, SourceMapOptions};
mod append {
    use string_wizard::{MagicStringOptions, UpdateOptions};
    use super::*;

    #[test]
    fn should_append_content() {
        let mut j = Joiner::default();

        let input = "<div>\n  hello, world\n</div>";
        let mut s1 = MagicString::with_options(input, MagicStringOptions{
            filename: Some("bar.js".to_string()),
        });
        let update_options = UpdateOptions {
            keep_original: true,
            ..Default::default()
        };
        s1.update_with(1, 2, "v", update_options.clone())
            .update_with(3, 4, "d", update_options.clone())
            .update_with(
                input.len() - 4,
                input.len() - 1,
                "h1",
                update_options.clone(),
            );

        let s2 = MagicString::with_options("import React from 'react';\n", MagicStringOptions{
            filename: Some("bar.js".to_string()),
        });

        j.append(s1);
        j.append(s2);

        let sm = j.source_map(SourceMapOptions {
            include_content: true,
        });

        assert_eq!(sm.mappings, "AAAA,CAACA,CAAC,CAACC,CAAC;AACJ;AACA,EAAEC,EAAG;AAFL;");
    }
}