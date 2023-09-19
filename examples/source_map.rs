use string_wizard::{MagicString, SourceMapOptions, UpdateOptions};

fn main() {
    let demo = "<div>
    hello, world
</div>";
    let mut s = MagicString::new(demo);

    // s.prepend("import React from 'react';\n")
    let update_options = UpdateOptions {
        store_name: true,
        ..Default::default()
    };
    s   
        .update_with(
            1,
            2,
            "v",
            update_options.clone(),
        )
        .update_with(
            3,
            4,
            "d",
            update_options.clone(),
        )
        .update_with(
            demo.len() - 4,
            demo.len() - 1,
            "h1",
            UpdateOptions {
                store_name: true,
                ..Default::default()
            },
        );

    let sm = s.source_map(SourceMapOptions {
        include_content: true,
    });

    std::fs::write("./demo.jsx.map", sm.to_string()).unwrap();
    std::fs::write("./demo.jsx", s.to_string()).unwrap();

    println!("{:#?}", s.to_string());
}
