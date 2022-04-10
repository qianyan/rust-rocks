use std::{fs::File, io::Write};

pub fn run() {
    let mut f = File::create("/tmp/test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello ").unwrap();
    // 不能通过编译，因为 by_ref() 返回了的 Self 类型是受限于 Sized 的，需要在编译期间确定大小。
//    let w1 = w.by_ref();
//    w1.write_all(b"world").unwrap();
}
