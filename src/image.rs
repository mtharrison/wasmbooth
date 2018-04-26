use std::slice;
use pixel::Pixel;

pub struct Image<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a mut [Pixel],
}

impl<'a> Image<'a> {
    pub fn from_raw(ptr: i32, width: usize, height: usize) -> Image<'a> {
        let num_pixels = width * height;
        let pixels = unsafe { slice::from_raw_parts_mut(ptr as *mut Pixel, num_pixels) };

        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn flip_x(&mut self) {
        for i in 0..self.pixels.len() {
            let (row, col) = self.index_to_row_col(i);
            if col >= self.width / 2 {
                let target_idx = self.row_col_to_index(row, self.width - 1 - col);
                let temp = self.pixels[i].clone();
                self.pixels[i] = self.pixels[target_idx];
                self.pixels[target_idx] = temp;
            }
        }
    }

    pub fn row_col_to_index(&self, row: usize, col: usize) -> usize {
        return (self.width * row) + col;
    }

    pub fn index_to_row_col(&self, i: usize) -> (usize, usize) {
        (i / self.width, i % self.width)
    }

    pub fn get_neighbour_colours(&self, i: usize) -> ([u8; 9], [u8; 9], [u8; 9]) {
        let mut red = [0; 9];
        let mut green = [0; 9];
        let mut blue = [0; 9];

        let (row, col) = self.index_to_row_col(i);

        let mut idx = 0;

        for i in row as isize - 1..row as isize + 2 {
            for j in col as isize - 1..col as isize + 2 {
                if i < 0 || i >= self.height as isize || j < 0 || j >= self.width as isize {
                    red[idx] = 0;
                    green[idx] = 0;
                    blue[idx] = 0;
                } else {
                    let pix = self.pixels[(self.width * (i as usize)) + (j as usize)];
                    red[idx] = pix.red;
                    green[idx] = pix.green;
                    blue[idx] = pix.blue;
                }
                idx += 1;
            }
        }

        return (red, green, blue);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut pixels = vec![
            Pixel::white(),
            Pixel::white(),
            Pixel::white(),
            //
            Pixel::white(),
            Pixel::red(),
            Pixel::white(),
            //
            Pixel::white(),
            Pixel::white(),
            Pixel::white(),
        ];

        let image = Image {
            width: 3,
            height: 3,
            pixels: &mut pixels[..],
        };

        assert_eq!(
            image.get_neighbour_colours(1),
            (
                [0, 0, 0, 255, 255, 255, 255, 255, 255],
                [0, 0, 0, 255, 255, 255, 255, 0, 255],
                [0, 0, 0, 255, 255, 255, 255, 0, 255],
            )
        );

        assert_eq!(
            image.get_neighbour_colours(4),
            (
                [255, 255, 255, 255, 255, 255, 255, 255, 255],
                [255, 255, 255, 255, 0, 255, 255, 255, 255],
                [255, 255, 255, 255, 0, 255, 255, 255, 255],
            )
        );

        assert_eq!(
            image.get_neighbour_colours(8),
            (
                [255, 255, 0, 255, 255, 0, 0, 0, 0],
                [0, 255, 0, 255, 255, 0, 0, 0, 0],
                [0, 255, 0, 255, 255, 0, 0, 0, 0],
            )
        );

        // assert_eq!(image.get_neighbours(8), [85, 255, 0, 255, 255, 0, 0, 0, 0]);
    }
}
