// 在 Rust 中，除非显式地做
// Box::leak()/Box::into_raw()/ManualDrop，堆内存的生命周期会默认和其栈内存的生命周期绑定在一起。
