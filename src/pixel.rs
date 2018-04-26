#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    _alpha: u8,
}

impl Pixel {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red,
            green,
            blue,
            _alpha: 255,
        }
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.red = r;
        self.green = g;
        self.blue = b;
    }

    pub fn set_gray(&mut self, g: u8) {
        self.red = g;
        self.green = g;
        self.blue = g;
    }

    pub fn invert(&mut self) {
        let (red, green, blue) = (255 - self.red, 255 - self.green, 255 - self.blue);
        self.set_rgb(red, green, blue);
    }

    pub fn grayscale(&mut self) {
        let avg = ((self.green as u32 + self.red as u32 + self.blue as u32) / 3) as u8;
        self.set_gray(avg);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rgb() {
        let pixel = Pixel::rgb(50, 100, 150);
        assert_eq!(pixel.red, 50);
        assert_eq!(pixel.green, 100);
        assert_eq!(pixel.blue, 150);
    }

    #[test]
    fn test_set_rgb() {
        let mut pixel = Pixel::rgb(50, 100, 150);
        assert_eq!(pixel.red, 50);
        assert_eq!(pixel.green, 100);
        assert_eq!(pixel.blue, 150);

        pixel.set_rgb(1, 2, 3);
        assert_eq!(pixel.red, 1);
        assert_eq!(pixel.green, 2);
        assert_eq!(pixel.blue, 3);
    }

    #[test]
    fn test_set_gray() {
        let mut pixel = Pixel::rgb(50, 100, 150);
        assert_eq!(pixel.red, 50);
        assert_eq!(pixel.green, 100);
        assert_eq!(pixel.blue, 150);

        pixel.set_gray(42);
        assert_eq!(pixel.red, 42);
        assert_eq!(pixel.green, 42);
        assert_eq!(pixel.blue, 42);
    }

    #[test]
    fn test_invert() {
        let mut pixel = Pixel::rgb(50, 100, 150);
        assert_eq!(pixel.red, 50);
        assert_eq!(pixel.green, 100);
        assert_eq!(pixel.blue, 150);

        pixel.invert();
        assert_eq!(pixel.red, 205);
        assert_eq!(pixel.green, 155);
        assert_eq!(pixel.blue, 105);
    }

    #[test]
    fn test_grayscale() {
        let mut pixel = Pixel::rgb(50, 100, 150);
        assert_eq!(pixel.red, 50);
        assert_eq!(pixel.green, 100);
        assert_eq!(pixel.blue, 150);

        pixel.grayscale();
        assert_eq!(pixel.red, 100);
        assert_eq!(pixel.green, 100);
        assert_eq!(pixel.blue, 100);
    }
}
