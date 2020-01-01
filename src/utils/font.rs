use std::collections::HashMap;
const ASCII_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

#[derive(Clone, Debug)]
pub struct FontBitmap {
    pub data: Vec<u8>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    font_size: f32,
    max_width: f32,
    max_height: f32,
}

#[derive(Debug, Clone)]
pub struct UvPosition {
    pub x: [f32; 2],
    pub y: [f32; 2],
}

fn ceil(value: f64, scale: i8) -> f64 {
    let multiplier = 10f64.powi(scale as i32) as f64;
    (value * multiplier).ceil() / multiplier
}

impl FontBitmap {
    pub fn get_uv_position(&self) -> UvPosition {
        let padding = ceil((self.font_size / 100.0).into(), 2) as f32;

        let x_start_position = ceil(((self.x - padding) / self.max_width) as f64, 3) as f32;
        let x_end_position = ceil(
            ((self.x + self.width + padding) as f32 / self.max_width) as f64,
            3,
        ) as f32;

        let y_start_position = ceil(((self.y - padding) / self.max_height) as f64, 3) as f32;

        let y_end_position = ceil(
            ((self.y + self.height + padding) / self.max_height) as f64,
            3,
        ) as f32;
        return UvPosition {
            x: [x_start_position, x_end_position],
            y: [y_start_position, y_end_position],
        };
    }
}

/// truetype font
pub struct FontPallet {
    font_data: &'static [u8],
    size: i32,
    pub max_w: i32,
    pub max_h: i32,
    cur_pt: cgmath::Point2<i32>,
    pub characters: HashMap<char, FontBitmap>,
}

impl FontPallet {
    /// parse a truetype file from bytes
    pub fn new(size: i32, font_data: &'static [u8]) -> Self {
        let (max_w, max_h) = (size * size as i32, size * size as i32);

        Self {
            font_data: &font_data,
            size: size,
            characters: HashMap::new(),
            cur_pt: cgmath::Point2::new(0, 0),
            max_h,
            max_w,
        }
    }

    /// manually cache characters
    pub fn cache(&mut self, s: &str) -> Self {
        let mut font = fontdue::Font::from_bytes(self.font_data).unwrap();

        let (max_height, max_width) = self.character_offsets(s);

        for ch in s.chars() {
            if !self.characters.contains_key(&ch) {
                let (metrics, bitmap) = font.rasterize(ch, self.size as f32);
                let (mut w, h) = (metrics.width as i32, metrics.height as i32);
                let (mut x, mut y) = self.cur_pt.into();
                let (mut offset_x, mut offset_y) = (0, 0);

                //Add offsets to smaller characters
                if metrics.width < max_width as usize {
                    offset_x = max_width - metrics.width as i32;
                }
                if metrics.height < max_height as usize {
                    offset_y = max_height - metrics.height as i32;
                }

                //Add space char width
                if ch == ' ' {
                    w = self.size / 3
                }

                //Put texture to new row on atlas, because current row is full
                if x + w >= self.max_w {
                    x = 0;
                    y += h + self.size as i32;
                }

                if y >= self.max_h {
                    println!("Error, font texture too high");
                }

                //Add new character to bitmap
                self.characters.insert(
                    ch,
                    FontBitmap {
                        data: bitmap,
                        y: y as f32,
                        x: x as f32,
                        width: w as f32,
                        height: h as f32,
                        offset_x: offset_x as f32,
                        offset_y: offset_y as f32,
                        font_size: self.size as f32,
                        max_width: self.max_w as f32,
                        max_height: self.max_h as f32,
                    },
                );

                //Move character forward on texture atlas
                x += w + self.size as i32;
                self.cur_pt = cgmath::Point2::new(x, y);
            }
        }

        FontPallet {
            size: self.size,
            font_data: self.font_data,
            characters: self.characters.clone(),
            cur_pt: self.cur_pt,
            max_h: self.max_h,
            max_w: self.max_w,
        }
    }

    //Get character offsets for correct alignment
    pub fn character_offsets(&mut self, s: &str) -> (i32, i32) {
        let (mut max_height, mut max_width) = (0, 0);
        let mut font = fontdue::Font::from_bytes(self.font_data).unwrap();
        s.chars().into_iter().for_each(|ch| {
            let (metrics, _bitmap) = font.rasterize(ch, self.size as f32);
            if metrics.height > max_height {
                max_height = metrics.height;
            }
            if metrics.width > max_width {
                max_width = metrics.width;
            }
        });
        return (max_height as i32, max_width as i32);
    }

    /// cache all ascii chars
    pub fn cache_asciis(&mut self) -> Self {
        return self.cache(ASCII_CHARS);
    }

    pub fn get(&self, ch: char) -> &FontBitmap {
        return self.characters.get(&ch).unwrap();
    }
}
