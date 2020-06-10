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
        width:config::config::WIN_WIDTH,
        height:config::config::WIN_HEIGHT
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
    let super_man_image1: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("super_man1.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let super_man_image2: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("super_man2.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let enemy_image1: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("enemy1.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let enemy_image2: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("enemy2.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let bullet1_logo: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        // &assets.join("bullet2.png"),
        &assets.join("bullet1.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let bullet2_logo: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("bullet2.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let explode_logo1: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("explode1.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let explode_logo2: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join("explode2.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    unsafe{
        //初始化加载子弹图片
        config::config::SUPER_MAN_TEXTURE1 = Option::Some(super_man_image1);
        config::config::SUPER_MAN_TEXTURE2 = Option::Some(super_man_image2);
        config::config::BULLET_TEXTURE_LEVEL1 = Option::Some(bullet1_logo);
        config::config::BULLET_TEXTURE_LEVEL2 = Option::Some(bullet2_logo);
        config::config::ENEMY_TEXTURE1 = Option::Some(enemy_image1);
        config::config::ENEMY_TEXTURE2 = Option::Some(enemy_image2);
        config::config::EXPLODE_TEXTURE1 = Option::Some(explode_logo1);
        config::config::EXPLODE_TEXTURE2 = Option::Some(explode_logo2);
    }


    //创建自己的超人
    let super_man = person::super_man::SuperMan::new(config::config::PERSON_SIZE, person::super_man::Category::type1);
    ///游戏实体
    let mut game = Game::new(vec!(super_man));

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
