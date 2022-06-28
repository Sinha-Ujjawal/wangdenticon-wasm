use image::{Rgb, RgbImage};

const MIDDLE_TILES: &'static [u8] = &[0, 1, 4, 5, 10, 11, 14, 15];
const OPPOSITE_MAP: &'static [u8] = &[0, 1, 8, 9, 4, 5, 12, 13, 2, 3, 10, 11, 6, 7, 14, 15];

fn render_tile(
    tile: u8,
    img_buffer: &mut RgbImage,
    grid_idx: u16,
    gridsize: u8,
    fgcolor: [u8; 3],
    bgcolor: [u8; 3],
) {
    let (row, col) = cell_index(grid_idx, gridsize);
    let m = tile % 16;

    let north = m & 1;
    let east = m & 2;
    let south = m & 4;
    let west = m & 8;

    // prefill with bgcolor
    for i in 0..3 {
        for j in 0..3 {
            img_buffer.put_pixel(col + j, row + i, Rgb(bgcolor))
        }
    }

    if north == 1 {
        for j in 0..3 {
            img_buffer.put_pixel(col + j, row, Rgb(fgcolor))
        }
    }

    if east == 2 {
        for i in 0..3 {
            img_buffer.put_pixel(col + 2, row + i, Rgb(fgcolor))
        }
    }

    if south == 4 {
        for j in 0..3 {
            img_buffer.put_pixel(col + j, row + 2, Rgb(fgcolor))
        }
    }

    if west == 8 {
        for i in 0..3 {
            img_buffer.put_pixel(col, row + i, Rgb(fgcolor))
        }
    }
}

fn cell_index(grid_idx: u16, gridsize: u8) -> (u32, u32) {
    let row = grid_idx / (gridsize as u16);
    let col = grid_idx % (gridsize as u16);
    (row as u32 * 3, col as u32 * 3)
}

pub fn generate(name: &str, gridsize: u8, invert: bool, size: usize) -> String {
    let hex_list = md5::compute(name).0;
    let hash_color = [hex_list[0], hex_list[1], hex_list[2]];
    let black = [0, 0, 0];
    let middle_tile = MIDDLE_TILES[hex_list[15] as usize % MIDDLE_TILES.len()];
    let (fgcolor, bgcolor) = if invert {
        (black, hash_color)
    } else {
        (hash_color, black)
    };
    let width = 3 * (gridsize as u32);
    let height = width;
    let mut img_buffer = RgbImage::new(width, height);
    let xub = (gridsize >> 1) + (gridsize & 1);
    for y in 0..(gridsize as usize) {
        for x in 0..xub {
            let left_idx = (y as u16 * gridsize as u16) + x as u16;
            let right_idx = y as u16 * gridsize as u16 + gridsize as u16 - 1 - x as u16;
            if left_idx != right_idx {
                let tile = hex_list[(y as u16 * xub as u16 + x as u16) as usize % 15];
                render_tile(tile, &mut img_buffer, left_idx, gridsize, fgcolor, bgcolor);
                render_tile(
                    OPPOSITE_MAP[tile as usize % OPPOSITE_MAP.len()],
                    &mut img_buffer,
                    right_idx,
                    gridsize,
                    fgcolor,
                    bgcolor,
                );
            } else {
                render_tile(
                    middle_tile,
                    &mut img_buffer,
                    left_idx,
                    gridsize,
                    fgcolor,
                    bgcolor,
                );
            }
        }
    }

    let mut buf = vec![];
    image::DynamicImage::ImageRgb8(img_buffer)
        .resize(size as u32, size as u32, image::imageops::Nearest)
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .expect("Writing image as png failed!");
    let res_base64 = base64::encode(&buf);
    res_base64
}
