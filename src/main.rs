#![deny(missing_docs)]

//! my super man

extern crate piston_window;
extern crate find_folder;

use piston_window::{OpenGL, PistonWindow, Glyphs, G2dTexture, Texture, Flip, TextureSettings};
use std::task::Context;
pub use piston_window::*;
use crate::map::draw::Draw;

pub mod config;
pub mod map;
pub mod weapon;
fn main() {
    let opengl = OpenGL::V3_2;



    let main_window = map::game_window::MainWindow::new();
    let win_size = main_window.win_size;

    let size = Size{
        width:win_size.width,
        height:win_size.height
    };
    let mut window: PistonWindow = WindowSettings::new(
        "my test title", size)
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    //字体
    let mut glyphs:Glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    //图片
    let snake_head_logo: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("snake.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let apple_logo: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        // &assets.join("apple.png"),
        &assets.join("mongo.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let main_window = map::game_window::MainWindow::new();
    while let Some(e) = window.next(){
        if let Some(r) = e.render_args(){
            window.draw_2d(&e, |c, g, device| {
                clear(config::colour::WHITE, g);
                main_window.draw(&mut glyphs,c,g,device);
            });
        }
    }
}
