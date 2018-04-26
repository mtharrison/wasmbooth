use image::Image;
use pixel::Pixel;
use convolution::{apply_convolution, ConvolutionMatrix};

pub enum FilterType {
    Invert,
    MirrorX,
    MirrorY,
    Grayscale,
    Convolution(ConvolutionMatrix),
}

pub trait ImageFilterExt {
    fn filter(&mut self, FilterType);
}

impl<'a> ImageFilterExt for Image<'a> {
    fn filter(&mut self, filter: FilterType) {
        match filter {
            FilterType::MirrorX => mirror_x(self),
            FilterType::MirrorY => mirror_y(self),
            FilterType::Grayscale => grayscale(self),
            FilterType::Invert => invert(self),
            FilterType::Convolution(matrix) => convolution(self, matrix),
        }
    }
}

fn mirror_x(image: &mut Image) {
    for i in 0..image.pixels.len() {
        let mid = image.width / 2;
        let (row, col) = image.index_to_row_col(i);

        if col > mid {
            let copy_col = mid - (col - mid);
            let j = image.row_col_to_index(row, copy_col);
            image.pixels[i] = image.pixels[j].clone();
        }
    }
}

fn mirror_y(image: &mut Image) {
    for i in 0..image.pixels.len() {
        let mid = image.height / 2;
        let (row, col) = image.index_to_row_col(i);

        if row > mid {
            let copy_row = mid - (row - mid);
            let j = image.row_col_to_index(copy_row, col);
            image.pixels[i] = image.pixels[j].clone();
        }
    }
}

fn grayscale(image: &mut Image) {
    for i in 0..image.pixels.len() {
        image.pixels[i].grayscale();
    }
}

fn convolution(image: &mut Image, matrix: ConvolutionMatrix) {
    let mut pixels_copy: Vec<Pixel> = image.pixels.iter().cloned().collect();
    let original = Image {
        width: image.width,
        height: image.height,
        pixels: &mut pixels_copy[..],
    };

    for i in 0..image.pixels.len() {
        let (row, col) = image.index_to_row_col(i);
        if row > 0 && row < image.height - 1 && col > 0 && col < image.width - 2 {
            let (red_n, green_n, blue_n) = original.get_neighbour_colours(i);
            let red = apply_convolution(red_n, matrix);
            let green = apply_convolution(green_n, matrix);
            let blue = apply_convolution(blue_n, matrix);
            image.pixels[i] = Pixel::rgb(red, green, blue);
        }
    }
}

fn invert(image: &mut Image) {
    for i in 0..image.pixels.len() {
        image.pixels[i].invert();
    }
}
