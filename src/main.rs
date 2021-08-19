use std::fs::File;
use std::io::prelude::*;

pub(crate) const WIDTH: u16 = 640;
pub(crate) const HEIGHT: u16 = 480;

fn display_buffer(file: &mut File) -> std::io::Result<()> {
    let header = format!("P6\n{} {}\n255\n", WIDTH, HEIGHT);

    file.write(header.as_bytes())?;

    let data: [u8; 3] = [255, 0, 250];
    for _i in 0..WIDTH {
        for _n in 0..HEIGHT {
            file.write(&data)?;
        }
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("test.ppm")?;
        // Write a slice of bytes to the file
        display_buffer(&mut file)?;
    }

    Ok(())
}
