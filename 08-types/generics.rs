// 对所有用到的泛型函数的泛型参数进行展开，生成若干函数，这个过程被称为单态化
// 单态化的好处是既保留编程中多态的灵活性，又保证了没有任何效率的损失。
// 明显的坏处是编译速度慢。
fn id<T>(x: T) -> T {
    return x;
}

fn main() {
    let int = id(10);
    let string = id("Qian");
    println!("{}, {}", int, string)
}
