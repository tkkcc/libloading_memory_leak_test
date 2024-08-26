use std::sync::LazyLock;

static BIG: LazyLock<Vec<u8>> = LazyLock::new(|| vec![0u8; 1000_000_000]);

fn my_func() -> u32 {
    let x = BIG[0];
    0
}

fn main() {
    fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let lib = libloading::Library::new("target/debug/libguest.so")?;
            let func: libloading::Symbol<unsafe extern "C" fn() -> u32> = lib.get(b"my_func")?;
            Ok(func())
        }
    }
    for i in 0.. {
        // leak
        let out = call_dynamic();

        // not leak
        // let out = my_func();
        println!("Hello, world! {i} {out:?}");
    }
}
