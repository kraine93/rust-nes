bitflags! {
    pub struct MaskRegister: u8 {
        const GREYSCALE = 0b0000_0001;
        const LEFTMOST_8PXL_BACKGROUND = 0b0000_0010;
        const LEFTMOST_8PXL_SPRITE = 0b0000_0100;
        const SHOW_BACKGROUND = 0b0000_1000;
        const SHOW_SPRITES = 0b0001_0000;
        const EMPHASISE_RED = 0b0010_0000;
        const EMPHASISE_GREEN = 0b0100_0000;
        const EMPHASISE_BLUE = 0b1000_0000;
    }
}

pub enum Colour {
    Red,
    Green,
    Blue,
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0b00000000)
    }

    pub fn is_grayscale(&self) -> bool {
        self.contains(MaskRegister::GREYSCALE)
    }

    pub fn leftmost_8pxl_background(&self) -> bool {
        self.contains(MaskRegister::LEFTMOST_8PXL_BACKGROUND)
    }

    pub fn leftmost_8pxl_sprite(&self) -> bool {
        self.contains(MaskRegister::LEFTMOST_8PXL_SPRITE)
    }

    pub fn show_background(&self) -> bool {
        self.contains(MaskRegister::SHOW_BACKGROUND)
    }

    pub fn show_sprites(&self) -> bool {
        self.contains(MaskRegister::SHOW_SPRITES)
    }

    pub fn emphasise(&self) -> Vec<Colour> {
        let mut result = Vec::<Colour>::new();
        if self.contains(MaskRegister::EMPHASISE_RED) {
            result.push(Colour::Red);
        }
        if self.contains(MaskRegister::EMPHASISE_BLUE) {
            result.push(Colour::Blue);
        }
        if self.contains(MaskRegister::EMPHASISE_GREEN) {
            result.push(Colour::Green);
        }

        result
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}
