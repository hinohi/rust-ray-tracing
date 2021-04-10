fn cast_pixel(v: f64) -> u8 {
    if v < 0.0 {
        0
    } else if v > 1.0 {
        255
    } else {
        (v * 255.999).floor() as u8
    }
}

fn main() {
    let width = 256;
    let height = 256;

    println!("P3");  //
    println!("{} {}", width, height);
    println!("255");
    for y in (0..height).rev() {
        for x in 0..width {
            let r = x as f64 / (width as f64 - 1.0);
            let g = y as f64 / (height as f64 - 1.0);
            let b = 0.25;
            println!("{} {} {}", cast_pixel(r), cast_pixel(g), cast_pixel(b));
        }
    }
}
