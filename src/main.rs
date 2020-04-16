#![deny(missing_docs)]

//! my super man

extern crate piston_window;
extern crate find_folder;

use piston_window::{OpenGL, PistonWindow, Glyphs, G2dTexture, Texture, Flip, TextureSettings};
use std::task::Context;
pub use piston_window::*;
use crate::map::draw::Draw;
use crate::game::Game;

pub mod config;
pub mod map;
pub mod weapon;
pub mod person;
pub mod game;
fn main() {
    let opengl = OpenGL::V3_2;

    let size = Size{
        width:config::comm::WIN_WIDTH,
        height:config::comm::WIN_HEIGHT
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
    let super_man_image: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("super_man.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let enemy_image: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("snake.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let bullet_logo: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        // &assets.join("apple.png"),
        &assets.join("mongo.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let super_man = person::super_man::Super_man::new(config::comm::PERSON_SIZE, &super_man_image);


    unsafe{
        config::comm::BULLET_TEXTURE = Option::Some(bullet_logo);
    }

    ///游戏实体
    let mut game = Game::new(super_man);

    let mut enemy = person::enemy::Enemy::new(config::comm::PERSON_SIZE, &enemy_image);
    game.add_enemy(enemy);



    while let Some(e) = window.next(){
        game.run(&e);
        if let Some(r) = e.render_args(){
            window.draw_2d(&e, |c, g, device| {
                clear(config::colour::WHITE, g);
                game.draw(&mut glyphs,c,g,device);
            });
        }
    }
}
