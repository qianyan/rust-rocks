fn main() {
   // let r = local_ref();
   // println!("r: {:p}", r);
    string_copy_is_not_ok();
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

// # 所有权的含义：
// 在一个作用域下，同一时刻，一个值只能有一个所有者。
// 堆上内存的生存与死亡，跟栈上的所有者牢牢绑定。
// 所以栈上的所有者牢牢绑定了堆变量的生命周期，所以我们只需要关心调用栈的生命周期。

// mutable reference
fn multipel_mutable_refs() {
    let mut data = vec![1, 2, 3];

    // 注意这里的 data 和 data.iter_mut() 都是可变引用
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
// cannot borrow `arr` as mutable because it is also borrowed as immutable
fn main_not_ok() {
    let mut arr = vec![1, 2, 3];
    // immutable borrow occurs here
    let last = arr.last();
    // mutable borrow occurs here
    arr.push(4);
    // immutable borrow later used here
    println!("last: {:?}", last);
}

// 编译器优化，导致不可变引用的生命周期在打印调用完之前就结束了，这种能力叫做 NLL(Non-Lexical Lifetimes)
fn main_ok() {
    let mut arr = vec![1, 2, 3];

    // 这里用完就结束生命周期了。
    let last = arr.last();
    println!("last: {:?}", last);

    arr.push(4);
}

fn string_copy_is_not_ok() {
    let mut arr = vec![String::from("a"), String::from("b")];
    // cache the last item
    // error[E0507] move occurs because value has type `String`, which does not implement the `Copy` trait
    let last = *arr.last().unwrap();
    arr.push(String::from("c"));
    // consume previously stored last item
    println!("last: {:?}", last);
}
