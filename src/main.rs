#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_analyze']"]
extern {}

use std::slice;

#[no_mangle]
pub unsafe fn analyze(buf: *const f32, length: usize) {
    analyze_internal(slice::from_raw_parts(buf, length));
}

fn analyze_internal(buf: &[f32]) {
    println!("{} of {} samples has energy",
             buf.iter().fold(0, |count, sample| if *sample == 0.0 { count } else { count + 1 } ),
             buf.len());
}

// When exporting test, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
