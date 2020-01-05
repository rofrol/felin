use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

const ASCII_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

lazy_static! {
    static ref FONT_CACHE: Mutex<HashMap<String, FontPallet>> = { Mutex::new(HashMap::new()) };
}

static mut FONT: Vec<u8> = Vec::new();

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
            ((self.x + self.width + padding) as f32 / self.max_height) as f64,
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
#[derive(Debug, Clone)]
pub struct FontPallet {
    pub max_w: i32,
    pub max_h: i32,
    pub characters: HashMap<char, FontBitmap>,
}

impl FontPallet {
    /// parse a truetype file from bytes
    pub fn create_font(font_name: &str, size: i32) -> FontPallet {
        let font = FontPallet::cache(ASCII_CHARS, size);

        let mut map = FONT_CACHE.lock().expect("lock failed");
        map.insert(font_name.to_string(), font.clone());
        font
    }

    /// manually cache characters
    pub fn cache(s: &str, size: i32) -> Self {
        let (max_texture_w, max_texture_h) = (size * size as i32, size * size as i32);
        let mut font = unsafe { fontdue::Font::from_bytes(&FONT[..]).unwrap() };
        let mut cur_pt: cgmath::Point2<i32> = cgmath::Point2::new(0, 0);
        let (max_height, max_width) = FontPallet::character_offsets(size, s);
        let mut characters: HashMap<char, FontBitmap> = HashMap::new();

        for ch in s.chars() {
            let (metrics, bitmap) = font.rasterize(ch, size as f32);
            let (mut w, h) = (metrics.width as i32, metrics.height as i32);
            let (mut x, mut y) = cur_pt.into();
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
                w = size / 3
            }

            //Put texture to new row on atlas, because current row is full
            if x + w >= max_texture_w {
                x = 0;
                y += h + size as i32;
            }

            if y >= max_texture_w {
                println!("Error, font texture too high");
            }

            //Add new character to bitmap
            characters.insert(
                ch,
                FontBitmap {
                    data: bitmap,
                    y: y as f32,
                    x: x as f32,
                    width: w as f32,
                    height: h as f32,
                    offset_x: offset_x as f32,
                    offset_y: offset_y as f32,
                    font_size: size as f32,
                    max_width: max_texture_w as f32,
                    max_height: max_texture_h as f32,
                },
            );

            //Move character forward on texture atlas
            x += w + size as i32;
            cur_pt = cgmath::Point2::new(x, y);
        }

        FontPallet {
            characters,
            max_h: max_texture_h,
            max_w: max_texture_w,
        }
    }

    //Get character offsets for correct alignment
    pub fn character_offsets(size: i32, s: &str) -> (i32, i32) {
        let (mut max_height, mut max_width) = (0, 0);
        let mut font = unsafe { fontdue::Font::from_bytes(&FONT[..]).unwrap() };

        s.chars().into_iter().for_each(|ch| {
            let (metrics, _bitmap) = font.rasterize(ch, size as f32);
            if metrics.height > max_height {
                max_height = metrics.height;
            }
            if metrics.width > max_width {
                max_width = metrics.width;
            }
        });
        return (max_height as i32, max_width as i32);
    }

    pub fn get(&self, ch: char) -> &FontBitmap {
        return self.characters.get(&ch).unwrap();
    }

    pub fn load_font(path: &str) {
        let mut f = File::open(path).expect("failed to read font");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("failed to read file");
        unsafe {
            FONT = buffer;
        }
    }

    pub fn get_font(name: &str) -> FontPallet {
        let map = FONT_CACHE.lock().expect("lock failed");
        let font = map.get(name).expect("Failed to find font");
        font.clone()
    }
}
