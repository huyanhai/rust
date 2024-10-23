use std::{fs, io};

use crate::vec3::Vec3;

static FILE_NAME: &'static str = "dist/01.ppm";

pub fn run() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut content = String::from(format!("P3\n{image_width} {image_height} \n255\n"));

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let i = i as f64;
            let j = j as f64;
            let w = image_width as f64;
            let h = image_height as f64;

            // r->0-1 g->1-0 b
            let color = Vec3(i / w, j / h, 0.25);

            content.push_str(&color.to_rgb_string());
        }
    }

    fs::write(FILE_NAME, content.as_bytes())?;
    Ok(())
}
