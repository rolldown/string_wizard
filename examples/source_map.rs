use string_wizard::{MagicString, SourceMapOptions, UpdateOptions};

fn main() {
    let demo = "<div>\n  hello, world\n</div>";
    let mut s = MagicString::new(demo);

    let update_options = UpdateOptions {
        keep_original: true,
        ..Default::default()
    };
    s.prepend("import React from 'react';\n")
        .update_with(1, 2, "v", update_options.clone())
        .update_with(3, 4, "d", update_options.clone())
        .update_with(demo.len() - 4, demo.len() - 1, "h1", update_options.clone());

    let sm = s.source_map(SourceMapOptions {
        include_content: true,
        ..Default::default()
    });

    std::fs::write("./demo.map.json", sm.to_json_string().unwrap())
    .unwrap();
    std::fs::write("./demo.jsx", s.to_string()).unwrap();

    println!("{:#?}", s.to_string());
}
