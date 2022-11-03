use crate::rgba::Rgba;

pub struct Image {
    pub pixels: Vec<Rgba>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            pixels: vec![Rgba::new(0, 0, 0, 0); (width * height) as usize],
            width,
            height,
        }
    }
    pub fn get_pixel(&self, w: u32, h: u32) -> &Rgba {
        &self.pixels[(self.width * h + w) as usize]
    }
}

#[cfg(test)]
mod test_image {
    use super::*;

    #[test]
    fn get_pixel() {
        let width: u32 = 4;
        let height: u32 = 3;
        let pixels: Vec<Rgba> = vec![
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(0, 1, 2, 3),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
            Rgba::new(255, 255, 255, 255),
        ];
        let image = Image {
            pixels,
            width,
            height,
        };
        let pixel = image.get_pixel(2, 1);
        assert_eq!(pixel.r, 0);
        assert_eq!(pixel.g, 1);
        assert_eq!(pixel.b, 2);
        assert_eq!(pixel.a, 3);
    }
}
