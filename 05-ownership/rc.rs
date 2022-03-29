use std::rc::Rc;
use std::cell::RefCell;

// # 什么是所有权？
// 在一个作用域下，同一时刻，一个值只能有一个所有者。
// 在作用域内变量的访问权和控制权
// 所谓访问权就是访问变量的权利。一旦变量发生了所有权move，则变量在当前作用域无法再访问
// 所谓控制权就是拥有所有权的变量所在作用域结束，则变量被释放
//
// # 什么是借用？
// 借用都是“临时的所有权”，借用不能超过(outlive)值的生命周期。在 Rust 中用 & 或 &mut 表示。

// # Rust 如何创建不受栈内存控制的堆内存？
// Rust 主动撕开了一个口子，Box::leak() -> 从堆内存泄漏出去，不受栈内存控制。
// 静态检查：靠编译器保证代码符合所有权规则；
// 动态检查，通过Box::leak()
// 让堆内存拥有不受限的生命周期，然后在运行过程中，通过对引用计数的检查，保证这样的的堆内存最终会得到释放。
fn main() {
//    let a = Rc::new(1);
//    let b = a.clone();
//    let c = a.clone();
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:?}, node2: {:?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    let mut a = node3.borrow_mut();
    a.update_downstream(Rc::new(RefCell::new(node5)));
//    node3.borrow_mut().update_downstream(Rc::new(RefCell::new(node5)));

    println!("node1: {:?}, node2: {:?}", node1, node2);
}

fn borrow_with_ref_cell() {
    let data = RefCell::new(1);
    // 如果没有这个作用域，那么编译没有任何问题，但是在运行期间，会得到
    // already mutably borrowed: BorrowError
    {
        let mut v = data.borrow_mut();
        *v += 1;
    }

    println!("data: {:?}", data.borrow());
}

fn mut_borrow_immut_borrow() {
    let mut data = 1;
    let v = &mut data;
    *v += 1;
    // 注意这段代码和上面的 borrow_with_ref_cell 的不同
    // 这里没有加上作用域，原因是编译器把生命周期缩小到了 println! 之前
    // 所以编译器的生命周期和运行时的生命周期是不一样，运行时的生命周期检查
    // 应该是作用域粒度的。
    print!("data: {:?}", &data)
}

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

// 外部可变性和内部可变性的区别 见Jk borrow_with_ref_cell
// 1. 外部可用性：let mut 或者 &mut; 编译时，如果不符合规则，产生编译错误
// 2. 内部可变性：使用 Cell / RefCell；运行时，如果不符合规则，产生 panic 错误
