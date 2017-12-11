#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_hello_world']"]
extern "C" {}

#[no_mangle]
pub fn hello_world(x: isize, y: isize) ->  {
    println!("Hello World!");
    format!("{} - {}", x, y).as_str()
}

fn main() {
    /* Intentionally left blank */
}