mod gfx;
mod spectro;

use std::slice;

extern crate bresenham;
extern crate rustfft;

#[no_mangle]
pub unsafe fn add_one(ptr: *const u32) -> u32 {
    *ptr + 1
}

#[no_mangle]
pub fn create_surface(width: usize, height: usize) -> *mut gfx::Surface {
    Box::into_raw(Box::new(gfx::Surface::new(width, height)))
}

#[no_mangle]
pub unsafe fn surface_buf(surface: *mut gfx::Surface) -> *const u8 {
    (*surface).buf.as_ptr() as *const u8
}

#[no_mangle]
pub fn create_spectro(bufsize: usize) -> *mut spectro::Spectro {
    Box::into_raw(Box::new(spectro::Spectro::new(bufsize)))
}

#[no_mangle]
pub unsafe fn draw_spectro(spectro: *const spectro::Spectro,
                           surface: *mut gfx::Surface,
                           signal: *const f32, signal_len: usize) {
    (*spectro).draw(slice::from_raw_parts(signal, signal_len), &mut *surface)
}

// When exporting other functions explicitly, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
