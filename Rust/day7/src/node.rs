pub struct Node {
    names: String,
    weights: i64,
    childs: Option<Vec<Node>>,
}

impl From<Vec<String>> for Node {
    fn from(v: Vec<String>) -> Node {
        match v.as_slice() {
            &[name, weight] => Node {
                names: name,
                weights: weight.replace("(", "").replace(")", "").parse().expect("Could not parse weight"),
                childs: None,
            },
            //other should be the remaining stuff
            &[name, weight, other] => Node {
                names: name,
                weights: weight,
                childs: Some(other),
            },
            _ => panic!()
        }
    }
}