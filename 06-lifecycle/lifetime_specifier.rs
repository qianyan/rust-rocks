// 生命周期参数，描述的是参数与参数之间，参数和返回值之间的关系，并没有改变原有的生命周期。
fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);
    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);
}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

// expected named lifetime parameter
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
