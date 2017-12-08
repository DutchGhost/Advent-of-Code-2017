#[derive(Debug)]
pub struct node {
    name: String,
    weight: i64,
    childs: Option<Vec<String>>,
}

impl From<Vec<String>> for node {
    fn from(v: Vec<String>) -> node {
        let (head, tail) = v.as_slice().split_at(2);
        if tail.is_empty() {
            node {
                name: head[0].to_string(),
                weight: head[1].replace("(", "").replace(")", "").parse().expect(
                    "Could not parse weight",
                ),
                childs: None,
            }
        } else {
            node {
                name: head[0].to_string(),
                weight: head[1].replace("(", "").replace(")", "").parse().expect(
                    "Could not parse weight",
                ),
                childs: Some(tail.to_vec()),
            }
        }
    }
}
