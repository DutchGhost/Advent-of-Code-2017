#[macro_use]
extern crate intrusive_collections;

use intrusive_collections::linked_list::*;

const PUZZLE: usize = 316;

#[derive(Debug)]
struct thing {
    link: Link,
    value: usize,
}

intrusive_adapter!(TestAdapter = Box<thing>: thing{link: Link});

//we already have a pointer to a position, so why not keep it? :)
fn main() {
    
    let mut vec_of_things = Vec::with_capacity(50_000_000);

    for i in 1..50_000_001 {
        vec_of_things.push(Box::new(thing{link: Link::new(), value: i}));
    }
    
    let mut iter_of_things = vec_of_things.into_iter();

    let mut buff = LinkedList::new(TestAdapter::new());
    let first = Box::new(thing {link: Link::new(), value: 0});
    buff.push_front(first);
    {
        let mut c = buff.cursor_mut();
        c.move_next();

        let mut current_pos = 0;
        for i in 1..50_000_001 {
            let idx = (current_pos + PUZZLE) % i;
            {   
                //insert at idx + 1.
                if idx == current_pos {
                    c.insert_after(iter_of_things.next().unwrap());
                    c.move_next();
                    //println!("inserted {:?} without moving at all", c.get());
                }
                
                //if the index is bigger than the current pos
                else if idx > current_pos {
                    //println!("walking forwards from {:?}", c.get());
                    while current_pos != idx {
                        current_pos += 1;
                        c.move_next();
                    }
                    //println!("walked forward to: {:?}", c.get());
                    assert!(idx == current_pos);
                    c.insert_after(iter_of_things.next().unwrap());
                    c.move_next();
                    //println!("inserted: {:?}", c.get());
                }
                else if idx < current_pos {
                    //println!("walking backwards from {:?}", c.get());
                    while current_pos != idx {
                        current_pos -= 1;
                        c.move_prev();
                    }
                    //println!("walked back to {:?}", c.get());
                    assert!(idx == current_pos);
                    c.insert_after(iter_of_things.next().unwrap());
                    c.move_next();
                    //println!("inserted: {:?}", c.get());
                }
                else {
                    //panic!("WHAT");
                }
            }
            current_pos = idx + 1;
            
            if i % 1_000 == 0 {
                println!("{}", i);
            }
        }
    }
    

    // let mut current_pos = 0;

    // for i in 1..50_000_001 {
    //     let idx = (current_pos + PUZZLE) % i;
    //     buff.insert(idx + 1, i);
    //     if i % 1_000_000 == 0 {
    //         println!("{}", i);
    //     }
    //     current_pos = idx + 1;
    // }

    // let idx = buff.iter().position(|item| item == &0).unwrap();
    // let l = buff.len();
    // println!("part 1: {}", buff[(idx + 1) % l ]);

    // let mut ans = 0;
    // let mut nxt = 0;
    // for i in 1..50_000_001 {
    //     nxt = (nxt + PUZZLE) % i;
    //     if nxt == 0 {ans = i;}
    //     nxt += 1;
    // }
    // println!("part 2: {}", ans);
}
