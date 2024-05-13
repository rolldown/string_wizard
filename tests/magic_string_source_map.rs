use string_wizard::{MagicString, SourceMapOptions, UpdateOptions};

#[test]
fn basic() {
    let input = "<div>\n  hello, world\n</div>";
    let mut s = MagicString::new(input);
    let update_options = UpdateOptions {
        keep_original: true,
        ..Default::default()
    };
    s.update_with(1, 2, "v", update_options.clone())
        .update_with(3, 4, "d", update_options.clone())
        .update_with(
            input.len() - 4,
            input.len() - 1,
            "h1",
            update_options.clone(),
        );

    let sm = s.source_map(SourceMapOptions {
        include_content: true,
        ..Default::default()
    });

    assert_eq!(
        sm.to_json_string().unwrap(),
        "{\"version\":3,\"sources\":[\"\"],\"sourcesContent\":[\"<div>\\n  hello, world\\n</div>\"],\"names\":[\"d\",\"v\",\"div\"],\"mappings\":\"AAAA,CAACA,CAAC,CAACC,CAAC;AACJ;AACA,EAAEC,EAAG\"}"
    );

    s.prepend("import React from 'react';\n");
    let sm = s.source_map(SourceMapOptions {
        include_content: true,
        ..Default::default()
    });
    assert_eq!(
        sm.to_json_string().unwrap(),
        "{\"version\":3,\"sources\":[\"\"],\"sourcesContent\":[\"<div>\\n  hello, world\\n</div>\"],\"names\":[\"d\",\"v\",\"div\"],\"mappings\":\";AAAA,CAACA,CAAC,CAACC,CAAC;AACJ;AACA,EAAEC,EAAG\"}"
    );
}
