extern crate treexml;

mod read {
    mod document {
        use treexml::{Document, XmlVersion};

        #[test]
        fn no_xml_tag() {
            let doc_raw = r#"
            <root>
                <child></child>
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();

            assert_eq!(doc.version, XmlVersion::Version10);
            assert_eq!(doc.encoding, "UTF-8".to_owned());
        }

        #[test]
        fn empty() {
            let doc_raw = "";
            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }

        #[test]
        fn no_root_tag() {
            let doc_raw = r#"
            <?xml version="1.1" encoding="UTF-8"?>
            "#;

            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }
    }

    mod tags {
        use treexml::Document;

        #[test]
        fn self_closing() {
            let doc_raw = r#"
            <root>
                <child />
            </root>
            "#;

            let _ = Document::parse(doc_raw.as_bytes()).unwrap();
        }

        #[test]
        fn self_closing_no_space() {
            let doc_raw = r#"
            <root>
                <child/>
            </root>
            "#;

            let _ = Document::parse(doc_raw.as_bytes()).unwrap();
        }

        #[test]
        fn mismatched_close() {
            let doc_raw = r#"
            <root></not_root>
            "#;

            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }

        #[test]
        fn mismatched_case_close() {
            let doc_raw = r#"
            <root></ROOT>
            "#;

            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }

        #[test]
        fn unclosed_root() {
            let doc_raw = r#"
            <root>
            "#;

            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }
    }

    mod element {
        use treexml::{Document, Element, Error, Node};

        #[test]
        fn find_child_none() {
            let doc_raw = r#"
            <root></root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            assert_eq!(root.find_child(|t| t.name == "child"), None);
        }

        #[test]
        fn find_child_one() {
            let doc_raw = r#"
            <root>
                <child attr_a="1" />
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            let mut child = Element::new("child");
            child.attributes.insert("attr_a".to_owned(), "1".to_owned());

            assert_eq!(root.find_child(|t| t.name == "child"), Some(&child));
        }

        #[test]
        fn find_child_many() {
            let doc_raw = r#"
            <root>
                <child attr_a="1" />
                <child attr_a="2" />
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            let mut child = Element::new("child");
            child.attributes.insert("attr_a".to_owned(), "1".to_owned());

            assert_eq!(root.find_child(|t| t.name == "child"), Some(&child));
        }

        #[test]
        fn find_child_mut_one() {
            let doc_raw = r#"
            <root>
                <child attr_a="1" />
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let mut root = doc.root.unwrap();

            {
                let child = root.find_child_mut(|t| t.name == "child").unwrap();
                let attr_a = child.attributes.get_mut(&"attr_a".to_owned()).unwrap();
                *attr_a = "2".to_owned();
            }

            let mut child = Element::new("child");
            child.attributes.insert("attr_a".to_owned(), "2".to_owned());

            assert_eq!(root.find_child(|t| t.name == "child"), Some(&child));
        }

        #[test]
        fn filter_children() {
            let doc_raw = r#"
            <root>
                <child>1</child>
                <child>2</child>
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            let mut ch1 = Element::new("child");
            let mut ch2 = Element::new("child");
            ch1.children.push(Node::Text("1".to_owned()));
            ch2.children.push(Node::Text("2".to_owned()));

            let children: Vec<&Element> = root.filter_children(|t| t.name == "child").collect();
            let children_ref = vec![&ch1, &ch2];

            assert_eq!(children, children_ref);
        }

        #[test]
        fn filter_children_mut() {
            let doc_raw = r#"
            <root>
                <child></child>
                <child></child>
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let mut root = doc.root.unwrap();

            {
                let mut children: Vec<&mut Element> =
                    root.filter_children_mut(|t| t.name == "child").collect();
                children[0].children.push(Node::Text("4".to_owned()));
                children[1].children.push(Node::Text("5".to_owned()));
            }

            let mut ch1 = Element::new("child");
            let mut ch2 = Element::new("child");
            ch1.children.push(Node::Text("4".to_owned()));
            ch2.children.push(Node::Text("5".to_owned()));

            let children: Vec<&Element> = root.filter_children(|t| t.name == "child").collect();
            let children_ref = vec![&ch1, &ch2];

            assert_eq!(children, children_ref);
        }

        #[test]
        fn find() {
            let doc_raw = r#"
            <root>
                <a>
                    <deep>
                        <tree>
                            <leaf>1</leaf>
                        </tree>
                    </deep>
                </a>
                <child>2</child>
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            let mut leaf = Element::new("leaf");
            leaf.children.push(Node::Text("1".to_owned()));

            assert_eq!(root.find("a/deep/tree/leaf").unwrap(), &leaf);

            match root.find("z").expect_err("Should have errored") {
                Error::ElementNotFound { .. } => {}
                _ => panic!("Error should have been ElementNotFound"),
            }
        }

        #[test]
        fn find_value() {
            let doc_raw = r#"
            <root>
                <number>2</number>
                <word>hello</word>
            </root>
            "#;

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            assert_eq!(root.find_value("number").unwrap(), Some(2));

            assert_eq!(root.find_value("word").unwrap(), Some("hello".to_string()));

            let cant_parse = root.find_value::<i32>("word");
            println!("cant parse was {:?}", cant_parse);
            match cant_parse.expect_err("Should have errored") {
                Error::ValueFromStr { .. } => {}
                _ => panic!("Error should have been ValueFromStr"),
            }
        }
    }

    mod cdata {
        use treexml::{Document, Element, Node};

        #[test]
        fn transposed_exclamation() {
            let doc_raw = "<root><[!CDATA[data]]></root>";
            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }

        #[test]
        fn no_opening_square_bracket() {
            let doc_raw = "<root><!CDATA[data]]></root>";
            let _ = Document::parse(doc_raw.as_bytes()).expect_err("Should have errored");
        }

        fn get_cdata(element: &Element) -> String {
            let mut result = String::new();
            for node in &element.children {
                match node {
                    &Node::CData(ref t) => result.push_str(t.as_str()),
                    _ => {}
                }
            }
            result
        }

        #[test]
        fn plain_text() {
            let doc_raw = "<root><![CDATA[data]]></root>";

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            assert_eq!(get_cdata(&root), "data");
        }

        #[test]
        fn nested_tags() {
            let doc_raw = "<root><![CDATA[ <tag /> ]]></root>";

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            assert_eq!(1, root.children.len());
            assert_eq!(get_cdata(&root), " <tag /> ");
        }

        #[test]
        fn text_across_cdata() {
            let doc_raw = "<root>text<![CDATA[cdata]]>text</root>";

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            assert_eq!(get_cdata(&root), "cdata");
            assert_eq!(root.text(), Some("texttext".to_owned()));
        }
    }

    mod comment {
        use treexml::{Document, Element, Node};

        fn get_comment(element: &Element) -> String {
            let mut result = String::new();
            for node in &element.children {
                match node {
                    &Node::Comment(ref t) => result.push_str(t.as_str()),
                    _ => {}
                }
            }
            result
        }

        #[test]
        fn plain_text() {
            let doc_raw = "<root><!-- Comment --></root>";

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();
            let root = doc.root.unwrap();

            assert_eq!(get_comment(&root), " Comment ");
        }
    }

    mod complete {
        use treexml::{Document, Element, Node, XmlVersion};

        #[test]
        fn parse_document() {
            let doc_raw = r#"
            <?xml version="1.1" encoding="UTF-8"?>
            <root>
                <child attr_a="1">content</child>
                <child attr_a="2"></child>
                <!-- Comment -->
                <child attr_a="3" />
                <child attr_a="4"><![CDATA[foo]]></child>
            </root>
            "#;

            let mut root = Element::new("root");

            let mut c1 = Element::new("child");
            c1.attributes.insert("attr_a".to_owned(), "1".to_owned());
            c1.children.push(Node::Text("content".to_owned()));

            let mut c2 = Element::new("child");
            c2.attributes.insert("attr_a".to_owned(), "2".to_owned());

            let mut c3 = Element::new("child");
            c3.attributes.insert("attr_a".to_owned(), "3".to_owned());

            let mut c4 = Element::new("child");
            c4.attributes.insert("attr_a".to_owned(), "4".to_owned());
            c4.children.push(Node::CData("foo".to_owned()));

            root.children.push(Node::Element(c1));
            root.children.push(Node::Element(c2));
            root.children.push(Node::Comment(" Comment ".to_string()));
            root.children.push(Node::Element(c3));
            root.children.push(Node::Element(c4));

            let doc_ref = Document {
                version: XmlVersion::Version11,
                encoding: "UTF-8".to_owned(),
                root: Some(root),
            };

            let doc = Document::parse(doc_raw.as_bytes()).unwrap();

            assert_eq!(doc, doc_ref);
        }
    }
}
