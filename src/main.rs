pub mod arithmetic;

use arithmetic::multiplication::extern_multiply;

fn main() {
    match extern_multiply(6, 7) {
        Ok(result) => println!("{}", result),
        Err(_) => println!("Uh oh!")
    }
}

