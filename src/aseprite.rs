use macroquad::prelude::*;
use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct AsepriteData {
    frames: HashMap<String, FrameData>,
    meta: MetaData,
}

#[derive(Debug, Deserialize)]
struct FrameData {
    frame: AsepriteRect,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: AsepriteRect,
    #[serde(rename = "sourceSize")]
    source_size: Size,
    duration: i32,
}

#[derive(Debug, Deserialize)]
struct AsepriteRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Debug, Deserialize)]
struct MetaData {
    app: String,
    version: String,
    image: String,
    format: String,
    size: Size,
    scale: String,
    #[serde(rename = "frameTags")]
    frame_tags: Vec<FrameTag>,
    layers: Vec<Layer>,
    slices: Vec<String>, // This is empty in the provided JSON, so we'll use Vec<String> for simplicity
}

#[derive(Debug, Deserialize)]
struct Size {
    w: i32,
    h: i32,
}

#[derive(Debug, Deserialize)]
struct FrameTag {
    name: String,
    from: i32,
    to: i32,
    direction: String,
    color: String,
}

#[derive(Debug, Deserialize)]
struct Layer {
    name: String,
    opacity: i32,
    #[serde(rename = "blendMode")]
    blend_mode: String,
}

struct Animation {
    texture: Texture2D,
    frames: Vec<FrameData>,
    current_frame: usize,
    frame_time: f32,
}

impl Animation {
    async fn new(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = fs::read_to_string(json_path)?;
        let aseprite_data: AsepriteData = serde_json::from_str(&json_data)?;
        
        let texture = load_texture(&format!("assets/{}", aseprite_data.meta.image)).await?;

        // Convert HashMap to Vec, sorted by frame number
        let mut frames: Vec<_> = aseprite_data.frames.into_iter().collect();
        frames.sort_by(|(a, _), (b, _)| {
            let a_num: i32 = a.split_whitespace().last().unwrap().trim_end_matches(".ase").parse().unwrap();
            let b_num: i32 = b.split_whitespace().last().unwrap().trim_end_matches(".ase").parse().unwrap();
            a_num.cmp(&b_num)
        });
        let frames = frames.into_iter().map(|(_, frame_data)| frame_data).collect();

        Ok(Animation {
            texture,
            frames,
            current_frame: 0,
            frame_time: 0.0,
        })
    }

    fn update(&mut self, dt: f32) {
        self.frame_time += dt;
        if self.frame_time >= self.frames[self.current_frame].duration as f32 / 1000.0 {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.frame_time = 0.0;
        }
    }

    fn draw(&self, x: f32, y: f32) {
        let frame = &self.frames[self.current_frame];
        let source_rect = Rect::new(
            frame.frame.x as f32,
            frame.frame.y as f32,
            frame.frame.w as f32,
            frame.frame.h as f32,
        );
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                ..Default::default()
            },
        );
    }
}

#[macroquad::main("Aseprite Animation")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut animation = Animation::new("assets/skeletron.json").await?;

    loop {
        clear_background(Color::new(0.1, 0.2, 0.3, 1.0));

        animation.update(get_frame_time());
        animation.draw(screen_width() / 2.0, screen_height() / 2.0);

        next_frame().await;
    }
}