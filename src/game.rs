//!游戏要素
use super::person::{enemy,super_man};
use super::weapon::{bullet};
use crate::config::comm;
use piston_window::*;
use crate::weapon::bullet::Bullet;
use std::ops::Deref;


///游戏要素
pub struct Game<'a>{
    ///超人
    pub super_man:super_man::Super_man<'a>,
    ///敌人
    enemys:Vec<enemy::Enemy<'a>>,
    ///子弹
    bullets:Vec<bullet::Bullet>,
}

///game operator
impl<'a> Game<'a>{

    ///new Game
    pub fn new(super_man:super_man::Super_man<'a>)-> Game<'a>{
        Game{
            super_man,
            enemys:Vec::new(),
            bullets:Vec::new(),
        }
    }

    ///创建一个敌人
    pub fn add_enemy(& mut self,enemy:enemy::Enemy<'a>){
        self.enemys.push(enemy);
    }

    ///增加一个子弹
    pub fn add_bullet(&mut self, bullet:bullet::Bullet){
        self.bullets.push(bullet);
    }

    ///运动
    pub fn run(&mut self,e:&Event){
        self.super_man.exec(e);

        let mut remove_index_list:Vec<usize> = Vec::new();
        for (index,enemy) in self.enemys.iter_mut().enumerate(){
            let bullet_opt:Option<Bullet> = enemy.run(self.super_man.coordinate);
            match bullet_opt {
                Some(bullet) => {
                    self.bullets.push(bullet);
                }
                None => {
                }
            }
            let (x,y) = enemy.coordinate;
            if x > comm::WIN_WIDTH || x < -1f64 || y > comm::WIN_HEIGHT || y < -1f64 {
                remove_index_list.push(index);
            };
        }
        for index in remove_index_list{
            self.enemys.remove(index);
        }


        let mut del_index_list = Vec::new();
        for (index,bullet) in self.bullets.iter_mut().enumerate(){
            let (x,y) = bullet.coordinate;
            if x > comm::WIN_WIDTH{
                del_index_list.push(index);
            }
            bullet.run();
        }
        for index in del_index_list{
            self.bullets.remove(index);
        }
    }

}


///画图
impl <'a> crate::map::draw::Draw for Game<'a>{

    ///画图
    fn draw (&self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        self.super_man.draw(glyphs, c, g, device);

        for enemy in self.enemys.iter(){
            enemy.draw(glyphs, c, g, device);
        }

        for bullet in self.bullets.iter(){
            bullet.draw(glyphs, c, g, device);
        }
    }
}