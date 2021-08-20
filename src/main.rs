use std::fs::File;
use std::io::prelude::*;

pub(crate) const WIDTH: u32 = 640;
pub(crate) const HEIGHT: u32 = 480;
const BUFF_SIZE: u32 = (WIDTH as u32 * HEIGHT as u32 * 3) as u32;

fn plot_pixel(buffer: &mut [u8], x: u32, y: u32, colour: u32, size: u32) {
    for numpixy in 0..size {
        for numpix in 0..size {
            let index = (x + numpix).clamp(0, WIDTH - 1) * 3
                + (y + numpixy).clamp(0, HEIGHT - 1) * WIDTH * 3;
            println!("{:?}", index);
            buffer[index as usize] = ((colour >> 16) & 0xff) as u8; //r
            buffer[(index + 1) as usize] = ((colour >> 8) & 0xff) as u8; //g
            buffer[(index + 2) as usize] = ((colour >> 0) & 0xff) as u8; //b
        }
    }
}

fn fill_buffer(buffer: &mut [u8], colour: u32) {
    let mut elem: i32 = 0;
    loop {
        buffer[elem as usize] = ((colour >> 16) & 0xff) as u8; //r
        buffer[(elem + 1) as usize] = ((colour >> 8) & 0xff) as u8; //g
        buffer[(elem + 2) as usize] = ((colour >> 0) & 0xff) as u8; //b
        elem += 3;
        if elem >= (buffer.len() - 3) as i32 {
            break;
        }
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
        fill_buffer(&mut buffer, 0xffffff);
        plot_pixel(&mut buffer, 320, 240, 0xff00FF, 8);
        display_buffer(&mut file, &buffer)?;
    }

    Ok(())
}
