fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
}
// wrong
fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    &a // return a reference to data owned by the current function
}

// ok
fn stack_to_heap_ok() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);
}

// wrong
fn stack_to_heap_wrong() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}

fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    data.push(&v); // borrowed value doest not live long enough
}

// 在一个作用域下，同一时刻，一个值只能有一个所有者。
// 所以栈上的所有者牢牢绑定了堆变量的生命周期，所以我们只需要关系调用栈的生命周期。

// mutable reference
fn multipel_mutable_refs() {
    let mut data = vec![1, 2, 3];

    for item in data.iter_mut() {
        data.push(*item + 1);
    }
}

// 为了保证内存安全，Rust 对可变引用也做了严格的约束
// 1. 在一个作用域下，仅仅允许一个活跃的可变引用；
// 2. 在一个作用域下，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。

// 可变引用没有实现 Copy trait，这是因为什么？
// 因为可变引用越多，在同一个作用域下是不安全的。

// 下面的代码该怎么才能通过编译？
fn main() {
    let mut arr = vec![1, 2, 3];
    let last = arr.last();
    arr.push(4);
    println!("last: {:?}", last);
}
// 编译器优化，导致不可变引用的生命周期在打印调用完之前就结束了，这种能力叫做 NLL(Non-Lexical Lifetimes)
