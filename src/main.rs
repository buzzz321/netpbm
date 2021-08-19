use std::fs::File;
use std::io::prelude::*;

pub(crate) const WIDTH: u16 = 640;
pub(crate) const HEIGHT: u16 = 480;
const BUFF_SIZE: u32 = (WIDTH as u32 * HEIGHT as u32 * 3) as u32;

fn fill_buffer(buffer: &mut [u8]) {
    for elem in buffer {
        *elem = 128;
    }
}

fn display_buffer(file: &mut File, buffer: &[u8; BUFF_SIZE as usize]) -> std::io::Result<()> {
    let header = format!("P6\n{} {}\n255\n", WIDTH, HEIGHT);

    file.write(header.as_bytes())?;
    file.write(buffer)?;

    Ok(())
}
fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("test.ppm")?;
        let mut buffer: [u8; BUFF_SIZE as usize] = [0; BUFF_SIZE as usize];
        // Write a slice of bytes to the file
        fill_buffer(&mut buffer);
        display_buffer(&mut file, &buffer)?;
    }

    Ok(())
}
