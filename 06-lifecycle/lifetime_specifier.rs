// 生命周期参数，描述的是参数与参数之间，参数和返回值之间的关系，并没有改变原有的生命周期。
// 所有使用了引用的函数都需要生命周期控制。
//
// 1. 所有引用类型的参数都有独立的生命周期 'a/'b 等。
// 2. 如果只有一个引用输入，他的生命周期会赋给所有输出。
// 3. 如果有多个引用类型的参数，其中一个是 self，则其生命周期会赋给所有输出。
//
// 数据结构的生命周期，要小于等于其内部字段的所有引用的生命周期。
fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);
    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);

    println!("first word of 'Hello world': {}", first("Hello world"));

    let s = "hello world";
    // 不可变引用发生了copy
    let mut s1 = s;
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

// expected named lifetime parameter
// 传入的参数的生命周期要大于等于(outlive)'a标注的生命周期。
// 原因是在max函数中，引用了 s1, s2，他们值的生命周期必须大于在函数中的生命周期
// 不然会出现 use afer free 这样的内存安全问题。
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

// Employee 的生命周期不能大于 name 和 title 两个字符的引用。
struct Employee<'a, 'b> {
    name: &'a str,
    title: &'b str,
    age: u8,
}
