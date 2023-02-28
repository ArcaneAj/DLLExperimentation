fn main() {
    match extern_multiply(2, 3) {
        Ok(result) => println!("{}", result),
        Err(_) => println!("Uh oh!")
    }
}

fn extern_multiply(x: i64, y: i64) -> Result<i64, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("CppArithmetic/x64/Release/CppArithmetic.dll")?;
        let func: libloading::Symbol<unsafe extern fn(x: i64, y: i64) -> i64> = lib.get(b"multiply")?;
        Ok(func(x, y))
    }
}