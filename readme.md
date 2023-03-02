Build CppArithmetic first
Then `cargo build --release` to be able to run the rust example that calls the cpp code
Then run `generate_headers.bat to enable the CppImport project to bind to the library
Then you can run CppImport to call the rust code that calls the cpp code