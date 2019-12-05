
use std::collections::HashMap;
const ASCII_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

#[derive(Clone, Debug)]
pub struct FontBitmap {
    pub data: Vec<u8>,
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    max_width: f32,
    max_height: f32,
}

#[derive(Debug, Clone)]
pub struct UvPosition {
    pub x: [f32; 2],
    pub y: [f32; 2],
}

impl FontBitmap {
    pub fn get_uv_position(&self) -> UvPosition {
        println!("{} {}", self.x / self.max_width, self.x / self.max_width);


        return UvPosition {
            x: [
                0.37,
                0.39,
            ],
            y: [
                0.08,
                0.1,
            ],
        };

        // return UvPosition {
        //     x: [
        //         0.5,
        //         0.51,
        //     ],
        //     y: [
        //         0.0,
        //         0.04,
        //     ],
        // };
    }
}

/// truetype font
pub struct FontPallet {
    font_data: &'static [u8],
    size: i32,
    pub max_w: i32,
    pub max_h: i32,
    cur_pt: cgmath::Point2<i32>,
    pub bitmaps: HashMap<char, FontBitmap>,
}

impl FontPallet {
    /// parse a truetype file from bytes
    pub fn new(size: i32, font_data: &'static [u8]) -> Self {
        let (max_w, max_h) = (size * 15, size * 15);

        Self {
            font_data: &font_data,
            size: size,
            bitmaps: HashMap::new(),
            cur_pt: cgmath::Point2::new(0, 0),
            max_h,
            max_w,
        }
    }

    /// manually cache characters
    pub fn cache(&mut self, s: &str) -> Self {
        let mut font = fontdue::Font::from_bytes(self.font_data).unwrap();
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
                        data: bitmap,
                        y: y as f32,
                        x: x as f32,
                        width: w as u32,
                        height: h as u32,
                        max_width: self.max_w as f32,
                        max_height: self.max_h as f32,
                    },
                );

                x += w;

                self.cur_pt = cgmath::Point2::new(x, y);
            }
        }

        FontPallet {
            size: self.size,
            font_data: self.font_data,
            bitmaps: self.bitmaps.clone(),
            cur_pt: self.cur_pt,
            max_h: self.max_h,
            max_w: self.max_w,
        }
    }

    /// cache all ascii chars
    pub fn cache_asciis(&mut self) -> Self {
        return self.cache(ASCII_CHARS);
    }

    pub fn get(&self, ch: char) -> &FontBitmap {
        return self.bitmaps.get(&ch).unwrap();
    }
}
