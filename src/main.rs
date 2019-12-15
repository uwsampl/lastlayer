// struct Register {
//     vid: u32,
//     path: String,
//     width: u32,
// }

// struct Memory {
//     vid: u32,
//     path: String,
//     width: u32,
// }

// use std::rc::Rc;

// #[derive(Debug)]
// enum ReadReg {
//     Equal(u32, u32),
//     Stmt(String, u32, u32),
//     Error(String),
//     If(Rc<ReadReg>, Rc<ReadReg>, Rc<ReadReg>),
// }

fn main() {
    println!("hello world\n");
    // let b = Switch::Cond(5 == 4);
    // println!("b:{:?}", &b);
}

// fn tree_weight_v1(t: BinaryTree) -> i32 {
//     match t {
//         BinaryTree::Leaf(payload) => payload,
//         BinaryTree::Node(left, payload, right) => {
//             tree_weight_v1(*left) + payload + tree_weight_v1(*right)
//         }
//     }
// }

// /// Returns tree that Looks like:
// ///
// ///      +----(4)---+
// ///      |          |
// ///   +-(2)-+      [5]
// ///   |     |
// ///  [1]   [3]
// ///
// fn sample_tree() -> BinaryTree {
//     let l1 = Box::new(BinaryTree::Leaf(1));
//     let l3 = Box::new(BinaryTree::Leaf(3));
//     let n2 = Box::new(BinaryTree::Node(l1, 2, l3));
//     let l5 = Box::new(BinaryTree::Leaf(5));

//     BinaryTree::Node(n2, 4, l5)
// }

// #[test]
// fn tree_demo_1() {
//     let tree = sample_tree();
//     assert_eq!(tree_weight_v1(tree), (1 + 2 + 3) + 4 + 5);
// }
