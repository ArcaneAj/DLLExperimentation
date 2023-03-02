pub fn extern_multiply(x: i64, y: i64) -> Result<i64, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("CppArithmetic.dll")?;
        let func: libloading::Symbol<unsafe extern fn(x: i64, y: i64) -> i64> = lib.get(b"multiply")?;
        Ok(func(x, y))
    }
}


#[no_mangle]
pub extern "C" fn multiply(x: i64, y: i64) -> i64 {
    match extern_multiply(x, y) {
        Ok(val) => val,
        Err(_) => -1
    }
}