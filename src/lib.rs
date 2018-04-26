mod bitflags;
mod filter;
mod image;
mod pixel;
mod convolution;

use bitflags::BitFlags;
use filter::FilterType;
use filter::ImageFilterExt;
use image::Image;
use pixel::Pixel;

#[no_mangle]
pub fn alloc_pixels(n: usize) -> i32 {
    let mut vec = Vec::<Pixel>::with_capacity(n);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Don't drop (JS is taking ownership)
    ptr as i32
}

#[no_mangle]
pub fn apply_filters(ptr: i32, options: u8, width: usize, height: usize) {
    let mut image = Image::from_raw(ptr, width, height);

    image.flip_x();

    let flags = BitFlags::new(options);

    if flags.get(0) {
        image.filter(FilterType::MirrorX);
    }

    if flags.get(1) {
        image.filter(FilterType::MirrorY);
    }

    if flags.get(2) {
        image.filter(FilterType::Grayscale);
    }

    if flags.get(3) {
        image.filter(FilterType::Convolution(convolution::EDGE_DETECT));
    }

    if flags.get(4) {
        image.filter(FilterType::Convolution(convolution::SHARPEN));
    }

    if flags.get(5) {
        image.filter(FilterType::Invert);
    }

    if flags.get(6) {
        image.filter(FilterType::Convolution(convolution::BLUR));
    }

    if flags.get(7) {
        image.filter(FilterType::Convolution(convolution::EMBOSS));
    }
}
