//see https://blog.thoughtram.io/references-in-rust/
fn main() {
    let data = vec![1, 2, 3, 4];
    let data_owned = &data;
    println!("addr of value: {:p}({:p}), addr of data {:p}, data_owned: {:p}, ptr of data: {:?}",
        &data, data_owned, &&data, &data_owned, *&data);

    println!("sum of data_owned: {}", sum(data_owned));

    // 堆上数据的地址是什么？
    println!("addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]);
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址不会发生改变，但是引用的地址发生了变化。
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x: &u32| acc + x)
}

//addr of value 指的是 data 这个胖指针的地址，他是堆上数据的所有者
//data_owned->data, &data_owned是自己的地址。
//addr of value: 0x7ff7bc8c4290(0x7ff7bc8c4290), addr of data 0x7ff7bc8c4350, data_owned: 0x7ff7bc8c42a8, ptr of data: [1, 2, 3, 4]
//addr of value: 0x7ff7bc8c4290, addr of ref: 0x7ff7bc8c40f0
//sum of data_owned: 10
//addr of items: [0x60000316c050, 0x60000316c054, 0x60000316c058, 0x60000316c05c]

