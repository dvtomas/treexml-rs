# `treexml`: An XML Tree Library for Rust

`treexml` is a simple element-tree style library for XML data.

## Usage

Like most `rust` packages, `treexml` uses [cargo](http://crates.io).
To use `treexml`, add the following to your project's `Cargo.toml`

This is a version modified by Tomáš Dvořák, currently available as a github dependency only. See issue #10 to see the reasons for modification.

```toml
[dependencies]
treexml = {git = "https://github.com/dvtomas/treexml-rs"}
```

The package exposes a crate named `treexml`.

```rust
extern crate treexml;
```

## Reading XML Data

```rust
extern crate treexml;

use treexml::Document;

fn main() {

    let doc_raw = r#"
    <?xml version="1.1" encoding="UTF-8"?>
    <table>
        <fruit type="apple">worm</fruit>
        <vegetable />
    </table>
    "#;

    let doc = Document::parse(doc_raw.as_bytes()).unwrap();
    let root = doc.root.unwrap();

    let fruit = root.find_child(|tag| tag.name == "fruit").unwrap().clone();
    println!("{} [{:?}] = {}", fruit.name, fruit.attributes, fruit.contents.unwrap());

}
```

## Writing XML Data

```rust
extern crate treexml;

use treexml::{Document, ElementBuilder as E};

fn main() {
    let mut something = E::new("something");
    root.attr("key", "value");
    root.text("some-text");

    let doc = Document::build(
        E::new("root").children(vec![
            E::new("list").children(vec![
                E::new("child").cdata("test data here"),
                E::new("child").attr("class", "foo").text("bar"),
                E::new("child").attr("class", 22).text(11),
                &mut E::new("child"),
                &mut something,
            ]),
        ])
    );

    println!("{}", doc);

}
```

## Contributing

This project is licensed under the MIT license.

If you encounter any issues, please file them on the GitHub issue tracker at https://github.com/rahulg/treexml/issues.
