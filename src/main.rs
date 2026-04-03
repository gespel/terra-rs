mod core;
use core::terra::Terra;

fn main() {
    let t = Terra::new("tester2-out.tif");
    t.render_debug_png("debug.png").unwrap();

    for y in 0..t.height {
        for x in 0..t.width {
            if let Some(value) = t.get_pixel(x, y) {
                println!("x: {}, y: {}, value: {}", x, y, value);
            }
        }
    }
}
