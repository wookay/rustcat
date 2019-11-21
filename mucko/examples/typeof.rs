// cargo run -q --example typeof

extern crate typename;
use typename::TypeName;
use std::ffi::{CString, CStr};

fn typename() {
    println!("{}", 10.type_name_of()); // i32
    println!("{}", 10.0.type_name_of()); // f64
    println!("{}", false.type_name_of()); // bool

    let word = "apple";
    println!("{}", word.type_name_of()); // str
    println!("{}", &word.type_name_of()); // str

    let cstring = CString::new(word).unwrap();
    // cstring  *const i8
    let ptr = cstring.as_ptr();
    println!("{:?}", ptr); // 0x7fd883402b50

    let cstr = unsafe { CStr::from_ptr(ptr) };
    println!("{:?}", cstr); // "apple"

    println!("{}", cstr.to_str().unwrap()); // apple
    match cstr.to_str() {
        Ok(s) => {
            println!("{}", s.type_name_of()); // str
            println!("{}", s); // apple
        }
        Err(_) => {
        }
    }
}

fn main() {
    typename();
}
