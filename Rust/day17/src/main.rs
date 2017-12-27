#[macro_use]
extern crate intrusive_collections;
use std::time::Instant;

use intrusive_collections::linked_list::*;

const PUZZLE: usize = 316;

#[derive(Debug)]
struct Node {
    link: Link,
    value: usize,
}

impl Node {
    fn get_value(&self) -> usize {
        self.value
    }
}

intrusive_adapter!(Linker = Box<Node>: Node{link: Link});

//we already have a pointer to a position, so why not keep it? :)
fn main() {
    
    let mut vec_of_nodes = Vec::with_capacity(50_000_000);

    for i in 1..50_000_001 {
        vec_of_nodes.push(Box::new(Node{link: Link::new(), value: i}));
    }
    
    let mut iter_of_nodes = vec_of_nodes.into_iter();

    let mut buff = LinkedList::new(Linker::new());
    let first = Box::new(Node {link: Link::new(), value: 0});
    buff.push_front(first);
    let time = Instant::now();
    {
        let mut c = buff.cursor_mut();
        c.move_next();

        let mut current_pos = 0;
        for i in 1..50_000_001 {
            let idx = (current_pos + PUZZLE) % i;
            {   
                //insert at idx + 1.
                if idx == current_pos {
                    c.insert_after(iter_of_nodes.next().unwrap());
                    c.move_next();
                }
                
                //if the index is bigger than the current pos,
                //move the cursor along witht the list
                else if idx > current_pos {
                    while current_pos != idx {
                        current_pos += 1;
                        c.move_next();
                    }
                    c.insert_after(iter_of_nodes.next().unwrap());
                    c.move_next();
                }
                //else, move the cursor down.
                else {
                    while current_pos != idx {
                        current_pos -= 1;
                        c.move_prev();
                    }
                    c.insert_after(iter_of_nodes.next().unwrap());
                    c.move_next();
                }
            }
            current_pos = idx + 1;
            
            if i % 100_000 == 0 {
                println!("{} {:?}", i, time.elapsed());
            }
        }
    }
    let mut iter = buff.iter();

    while let Some(thing) = iter.next() {
        if thing.get_value() == 0 {
            break;
        }
    }
    println!("part 2: {:?}", iter.next());
    println!("done in {:?} seconds", time.elapsed());
}
