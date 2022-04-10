use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

// compiling error
//impl<W: Write> MyWriter<W> {
//    pub fn new(url: &str) -> Self {
//        let stream = TcpStream::connect(url).unwrap();
//        Self {
//        // 逻辑上讲，BufWriter 实现了 Writer
//        trait，这个应该能通过编译，但是事实相反。我猜测这是Rust很严格参数多态需要实现单态化的限制。
//            writer: BufWriter::new(stream)
//        }
//    }
//
//    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
//        self.writer.write_all(buf.as_bytes())
//    }
//}

impl<W: Write> MyWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let bufWriter = BufWriter::new(stream);
    let mut writer = MyWriter::new(bufWriter);
    writer.write("Hello world!");
}
