use std::{borrow::Borrow, io::Write};

use ::png::{BitDepth, ColorType, Encoder};
use tiny_skia::Pixmap;

pub fn to_vec(pixmap: &Pixmap) -> anyhow::Result<Vec<u8>> {
    let mut data = Vec::new();
    write(&mut data, pixmap.width(), pixmap.height(), [Ok(pixmap)])?;

    Ok(data)
}

pub fn write<P: Borrow<Pixmap>>(
    output: impl Write,
    width: u32,
    height: u32,
    strips: impl IntoIterator<Item = anyhow::Result<P>>,
) -> anyhow::Result<()> {
    let mut encoder = Encoder::new(output, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    {
        let mut stream = writer.stream_writer()?;
        let mut row = vec![0; width as usize * 4];
        for strip in strips {
            let strip = strip?;
            for pixels in strip.borrow().pixels().chunks_exact(width as usize) {
                for (pixel, rgba) in pixels.iter().zip(row.chunks_exact_mut(4)) {
                    let color = pixel.demultiply();
                    rgba.copy_from_slice(&[
                        color.red(),
                        color.green(),
                        color.blue(),
                        color.alpha(),
                    ]);
                }
                stream.write_all(&row)?;
            }
        }
        stream.finish()?;
    }

    writer.finish()?;

    Ok(())
}
