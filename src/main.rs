use core::fmt;

#[derive(Debug)]
pub struct LinkedList<'a> {
    head: Option<Box<Node<'a>>>,
}

impl<'a> LinkedList<'a> {
    fn new(head: Option<Box<Node<'a>>>) -> Self {
        Self { head }
    }

    fn insert_value(&self, value: i32) -> LinkedList {
        let new_head = Box::new(Node {
            value,
            next: &self.head,
        });
        let new_linked_list = LinkedList {
            head: Some(new_head),
        };
        new_linked_list
    }

    fn to_string_rec_start(node: &Option<Box<Node>>) -> String {
        if node.is_none() {
            return "".to_owned();
        }
        let res = format!("{}", node.as_ref().unwrap().value);

        LinkedList::to_string_rec(node.as_ref().unwrap().next, res)
    }

    fn to_string_rec(node: &Option<Box<Node>>, res: String) -> String {
        if node.is_none() {
            return res;
        }
        let new_res = format!("{} -> {}", res, node.as_ref().unwrap().value);
        return LinkedList::to_string_rec(node.as_ref().unwrap().next, new_res);
    }

    fn to_string(&self) -> String {
        LinkedList::to_string_rec_start(&self.head)
    }
}

#[derive(Debug)]
struct Node<'a> {
    value: i32,
    next: &'a Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(new_value: i32) -> Self {
        Node {
            value: new_value,
            next: &None,
        }
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({})", self.value)
    }
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_to_string() {
//         let sample_list = LinkedList::new(None);
//     }
// }

impl<'a> fmt::Display for LinkedList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_write = "LinkedList(".to_owned();
        // let mut current = &self.head;
        let string_to_add = &self.to_string();
        println!("string_to_add: {}", string_to_add);
        to_write = format!("{}{}", to_write, string_to_add);
        // if current.is_some() {
        //     loop {
        //         to_write = format!("{}{}", to_write, current.as_ref().unwrap().value);
        //         current = current.as_ref().unwrap().next;
        //         if current.is_none() {
        //             break;
        //         }
        //         to_write = format!("{} -> ", to_write);
        //     }
        // }

        write!(f, "{})", to_write)
    }
}

fn main() {
    let sample_node = Node::new(14);
    let my_linked_list = LinkedList::new(None);
    println!("sample_node: {}", sample_node);
    println!("LinkedList: {}", my_linked_list);

    let new_linked_list = my_linked_list.insert_value(45);
    let new_linked_list2 = new_linked_list.insert_value(99);
    // let new_linked_list3 = new_linked_list2.insert_value(1432).insert_value(-123);
    println!("new_linked_list2: {}", new_linked_list2);
    println!("new_linked_list2: {:?}", new_linked_list2);
    // println!("new_linked_list3: {}", new_linked_list3);

    println!("bye");
}
