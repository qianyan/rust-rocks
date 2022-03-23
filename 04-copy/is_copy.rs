fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // function(actually a pointer) is Copy
    is_copy::<fn()>();

    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // immutable reference is Copy
    is_copy::<&[u8; 4]>();
    is_copy::<(&str, &str)>();

    // array /tuple with values that copy is Copy
    is_copy::<(u32, &str)>();
}

fn types_not_impl_copy_trait() {
    // unsized or dynamic sized type is not Copy
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();

    // mutable reference is not Copy
    is_copy::<&mut String>();

    // array / tuple with values that not Copy is not Copy
    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}

fn main() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}

// 原生类型，包括函数、不可变引用和裸指针实现了 Copy
// 数组和元组，如果内部的数据结构实现了 Copy，则他们也实现了 Copy
// 可变引用没有实现 Copy
// 非固定大小的数据结构，没有实现 Copy

// copy 语义和 drop 语义不兼容，因为 copy 发生在栈上，drop 发生在堆上
// 每个内存对象（无论堆还是栈），仅有一个所有者。当所有者超出了作用域，则对象将会被释放。
// clone 是 copy trait 的基类，clone 是开发者调用的，默认是深拷贝，需要用 drop trait
// 来保证内存不泄露。为了避免深拷贝，使用*borrow*概念。
// 没有实现 Copy trait 的数据结构，就会默认使用 Move 的语义。
