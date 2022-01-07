#[derive(Clone, Copy, Default)]
pub struct Color(pub u8, pub u8, pub u8);

pub const BLACK:Color = Color(0,0,0);


impl Color {
    pub fn with_alpha(self, a: u8) -> Color {
        let mut out = self.clone();
        // out.3 = a;
        out
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let (r, g, b) = (self.0 as u32, self.1 as u32, self.2 as u32);
        (r << 16) | (g << 8) | b
    }
}