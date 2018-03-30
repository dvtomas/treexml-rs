use super::*;

/// A builder for Element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementBuilder {
    element: Element,
}

impl ElementBuilder {
    /// Create a builder for an `Element` with the tag name `name`
    pub fn new<S: ToString>(name: S) -> ElementBuilder {
        ElementBuilder {
            element: Element::new(name),
        }
    }

    /// Set the element's prefix to `prefix`
    pub fn prefix<S>(&mut self, prefix: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.prefix = Some(prefix.to_string());
        self
    }

    /// Set the element's attribute `key` to `value`
    pub fn attr<K, V>(&mut self, key: K, value: V) -> &mut ElementBuilder
        where
            K: ToString,
            V: ToString,
    {
        self.element
            .attributes
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Adds a `text` child
    pub fn text<S>(&mut self, text: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.children.push(Node::Text(text.to_string()));
        self
    }

    /// Adds a `cdata` child
    pub fn cdata<S>(&mut self, cdata: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.children.push(Node::CData(cdata.to_string()));
        self
    }

    /// Adds a `comment` child
    pub fn comment<S>(&mut self, comment: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.children.push(Node::Comment(comment.to_string()));
        self
    }

    /// Append element children
    pub fn children(&mut self, children: Vec<&ElementBuilder>) -> &mut ElementBuilder {
        self.element.children.append(&mut children.into_iter().map(|builder| Node::Element(builder.element())).collect());
        self
    }

    /// Append element children
    pub fn children_elements(&mut self, children: Vec<Element>) -> &mut ElementBuilder {
        self.element.children.extend(children.into_iter().map(Node::Element));
        self
    }

    /// Append nodes
    pub fn children_nodes(&mut self, mut children: Vec<Node>) -> &mut ElementBuilder {
        self.element.children.append(&mut children);
        self
    }

    /// Clones an `Element` from the builder, consuming the builder
    pub fn element(&self) -> Element {
        self.element.clone()
    }

    /// Builds an `Element` from this builder
    pub fn build(self) -> Element {
        self.element
    }
}

impl From<ElementBuilder> for Element {
    fn from(value: ElementBuilder) -> Element {
        value.element()
    }
}

impl From<Element> for ElementBuilder {
    fn from(element: Element) -> ElementBuilder {
        ElementBuilder { element }
    }
}
