#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_test']"]
extern {}

#[no_mangle]
pub fn test() {
    println!("rust: test!");
}

// When exporting test, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
