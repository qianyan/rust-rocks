//see https://blog.thoughtram.io/references-in-rust/
use std::mem;
fn main() {
    let data = vec![1, 2, 3, 4];
    let data_owned = &data;
    // see &* meaning: https://stackoverflow.com/questions/41273041/what-does-combined-together-do-in-rust
    // deref will recuirsivly deref, Vec<iu32> deref to &[T] and then to [T], so &*data deref to
    // &[T]
    println!("addr of value: {:p}({:p}), addr of data {:p}, data_owned: {:p}, ptr of data: {:?}",
    &data, data_owned, &*data, &data_owned, data);

    println!("sum of data_owned: {}", sum(data_owned));
     // 堆上数据的地址是什么？
    println!("addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]);

    //let p:[usize;3];
    let p = unsafe { mem::transmute::<&Vec<u32>, usize>(&data) }; 
    println!("stack value: {:x}",p); 


}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址不会发生改变，但是引用的地址发生了变化。
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x: &u32| acc + x)
}

fn inspect_mem() {
    let data:Vec<u32> = vec![10, 42, 9, 8]; 
    let p:[usize;3];
    p = unsafe { mem::transmute(data) }; 
    println!("stack value: {:x}, {:x}, {:x}",p[0], p[1], p[2]); 

}

//addr of value 指的是 data 这个胖指针的地址，他是堆上数据的所有者
//data_owned->data, &data_owned是自己的地址。
//
// addr of value: 0x7ff7bbc7d198(0x7ff7bbc7d198), addr of data 0x7ff7bbc7d270, data_owned: 0x7ff7bbc7d1b0, ptr of data: [1, 2, 3, 4]
// addr of value: 0x7ff7bbc7d198, addr of ref: 0x7ff7bbc7cfc0
// sum of data_owned: 10
// addr of items: [0x600003ea4050, 0x600003ea4054, 0x600003ea4058, 0x600003ea405c]
// stack value: 7ff7bbc7d198
