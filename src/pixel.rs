#[derive(Clone, Copy)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    _alpha: u8,
}

impl Pixel {
    pub fn white() -> Pixel {
        Pixel {
            red: 255,
            green: 255,
            blue: 255,
            _alpha: 255,
        }
    }

    pub fn red() -> Pixel {
        Pixel {
            red: 255,
            green: 0,
            blue: 0,
            _alpha: 255,
        }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red,
            green,
            blue,
            _alpha: 255,
        }
    }

    pub fn invert(&mut self) {
        let (red, green, blue) = (255 - self.red, 255 - self.green, 255 - self.blue);
        self.set_rgb(red, green, blue);
    }

    pub fn grayscale(&mut self) {
        let avg = ((self.green as u32 + self.red as u32 + self.blue as u32) / 3) as u8;
        self.set_rgb(avg, avg, avg);
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.red = r;
        self.green = g;
        self.blue = b;
    }
}
