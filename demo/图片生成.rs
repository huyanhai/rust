use std::{fs, io};

static FILE_NAME: &'static str = "dist/00.ppm";

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut content = String::from(format!("P3\n{image_width} {image_height} \n255\n"));

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let i = i as f64;
            let j = j as f64;
            let w = image_width as f64;
            let h = image_height as f64;

            let r = (i / w) as f64;
            let g = (j / h) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            let str = format!("{ir} {ig} {ib}\n");
            content.push_str(&str);
        }
    }

    fs::write(FILE_NAME, content.as_bytes())?;
    Ok(())
}
