use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

impl Node {
    fn text(data: String) -> Node {
        Node { children: vec![], node_type: Text(data) }
    }

    fn elem(name: String, attrs: AttrMap, ch: Vec<Node>) -> Node {
        Node {
            children: ch,
            node_type: Element(ElementData {
                tag_name: name,
                attributes: attrs,
            })
        }

    }

    fn comment(data: String) -> Node {
        Node { children: vec![], node_type: Comment(data) }
    }
}
