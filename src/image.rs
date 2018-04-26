use std::slice;
use pixel::Pixel;

pub struct Image<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a mut [Pixel],
}

impl<'a> Image<'a> {
    pub fn from_raw(ptr: *mut Pixel, width: usize, height: usize) -> Image<'a> {
        let num_pixels = width * height;
        let pixels = unsafe { slice::from_raw_parts_mut(ptr, num_pixels) };

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
        (self.width * row) + col
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
                let pix = self.pixels[(self.width * (i as usize)) + (j as usize)];
                red[idx] = pix.red;
                green[idx] = pix.green;
                blue[idx] = pix.blue;
                idx += 1;
            }
        }

        (red, green, blue)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_raw() {
        let mut pixels = [
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(0, 0, 0),
        ];

        let image = Image::from_raw(&mut pixels[0], 2, 2);

        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        assert_eq!(image.pixels, pixels);
    }

    #[test]
    fn test_flip_x() {
        let mut pixels = [
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
        ];

        let mut image = Image::from_raw(&mut pixels[0], 2, 2);
        image.flip_x();

        assert_eq!(image.pixels, [
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100,0, 0),
        ]);
    }

    #[test]
    fn test_row_col_to_index() {
        let mut pixels = [
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
        ];

        let image = Image::from_raw(&mut pixels[0], 2, 2);

        assert_eq!(image.row_col_to_index(1, 0), 2);
        assert_eq!(image.row_col_to_index(1, 1), 3);
    }

    #[test]
    fn test_index_to_row_col() {
        let mut pixels = [
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
        ];

        let image = Image::from_raw(&mut pixels[0], 2, 2);

        assert_eq!(image.index_to_row_col(2), (1, 0));
        assert_eq!(image.index_to_row_col(3), (1, 1));
    }

    #[test]
    fn test_get_neighbour_colours() {
        let mut pixels = [
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
            Pixel::rgb(0, 0, 0),
            Pixel::rgb(100, 0, 0),
        ];

        let image = Image::from_raw(&mut pixels[0], 3, 3);

        assert_eq!(image.get_neighbour_colours(4), (
            [100, 0, 100, 0, 100, 0, 100, 0, 100],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ));
    }
}
