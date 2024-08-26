use std::sync::LazyLock;

static BIG: LazyLock<Vec<u8>> = LazyLock::new(|| vec![0u8; 1000_000_000]);

#[no_mangle]
extern "C" fn my_func() -> u32 {
    let x = BIG[0];
    0
}
