fn main() {
    let data = 1;
    let data_owned = data;
    println!("addr of value: {:p}({:?}), addr of data {:p}, data_owned: {:p}, ptr of data: {:?}",
        &data, data_owned, &&data, &data_owned, *&data);
}

//addr of value: 0x7ff7bc7253b0(1), addr of data 0x7ff7bc725470, data_owned: 0x7ff7bc7253b4, ptr of data: 1
