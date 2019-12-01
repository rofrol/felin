use std::collections::HashMap;

const ASCII_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

#[derive(Clone, Debug)]
pub struct FontBitmap {
    pub bitmap: Vec<u8>,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// truetype font
pub struct FontPallet {
    size: i32,
    max_w: i32,
    max_h: i32,
    cur_pt: cgmath::Point2<i32>,
    pub bitmaps: HashMap<char, FontBitmap>,
}

impl FontPallet {
    /// parse a truetype file from bytes
    pub fn new(size: i32) -> Self {
        
        let (max_w, max_h) = (size * 32, size * 32);

        Self {
            size: size,
            bitmaps: HashMap::new(),
            cur_pt: cgmath::Point2::new(0, 0),
            max_h,
            max_w,
        }
    }

    /// manually cache characters
    pub fn cache(&mut self, s: &str, font: &[u8]) -> Self {

        let mut font = fontdue::Font::from_bytes(font).unwrap();
        for ch in s.chars() {
            if !self.bitmaps.contains_key(&ch) {
                let (metrics, bitmap) = font.rasterize(ch, self.size as f32);
                let (w, h) = (metrics.width as i32, metrics.height as i32);
                let (mut x, mut y) = self.cur_pt.into();

                //Put texture to new row, because current row is full
                if x + w >= self.max_w {
                    x = 0;
                    y += h;
                }

                if y >= self.max_h {
                    println!("Error, font texture too high");
                }

                //Add new character to bitmap
                self.bitmaps.insert(
                    ch,
                    FontBitmap {
                        bitmap,
                        y: y as u32,
                        x: x as u32,
                        width: w as u32,
                        height: h as u32,
                    },
                );

                x += w;

                self.cur_pt = cgmath::Point2::new(x, y);
            }
        }

        FontPallet {
            size: self.size,
            bitmaps: self.bitmaps.clone(),
            cur_pt: self.cur_pt,
            max_h: self.max_h,
            max_w: self.max_w,
        }
    }

    /// cache all ascii chars
    pub fn cache_asciis(&mut self, font: &[u8]) -> Self {
        return self.cache(ASCII_CHARS, font)
    }

    pub fn get(&self, ch: char) -> &FontBitmap {
        return self.bitmaps.get(&ch).unwrap();
    }
}
