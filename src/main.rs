use std::io::{self, Read};
use serde::{Serialize, Deserialize};
use image;
use imageproc::{drawing};
use rusttype::{Font, Scale};

#[derive(Serialize, Deserialize)]
struct Watermark {
    text: String,
    left: u32,
    top: u32,
    font_size: f32,
}

const FONT_FILE : &[u8] = include_bytes!("PingFang-Bold.ttf") as &[u8];
const TEMPLATE_BUF : &[u8] = include_bytes!("bg.png") as &[u8];

fn main() {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).expect("Error reading from STDIN");
  let obj: FaasInput = serde_json::from_str(&buffer).unwrap();
  // println!("{} {}", &(obj.body)[..5], obj.body.len());

  let mut img = image::load_from_memory(TEMPLATE_BUF).unwrap();

  let memes: Vec<Watermark> = serde_json::from_str(&(obj.body)).unwrap();
  for m in memes {
    _watermark(m, &mut img);
  }

  let mut buf = vec![];
  img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
  println!("{}", base64::encode_config(buf, base64::STANDARD));
}

fn _watermark(w: Watermark, img: &mut image::DynamicImage) {
  let font_size = w.font_size;

  let font = Vec::from(FONT_FILE);
  let font = Font::try_from_vec(font).unwrap();

  let scale = Scale {
    x: font_size + 1.0,
    y: font_size + 1.0,
  };
  drawing::draw_text_mut(img, image::Rgba([0, 0, 0, 255u8]), w.left - 2, w.top - 2, scale, &font, &w.text);

  let scale = Scale {
    x: font_size,
    y: font_size,
  };
  drawing::draw_text_mut(img, image::Rgba([255u8, 255u8, 255u8, 255u8]), w.left, w.top, scale, &font, &w.text);
}


#[derive(Deserialize, Debug)]
struct FaasInput {
    body: String
}
