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
    ///超人发射的子弹
    super_man_shoot_bullets:Vec<bullet::Bullet>,
    ///敌人发射的子弹
    enemy_shoot_bullets:Vec<bullet::Bullet>,
}

///game operator
impl<'a> Game<'a>{

    ///new Game
    pub fn new(super_man:super_man::Super_man<'a>)-> Game<'a>{
        Game{
            super_man,
            enemys:Vec::new(),
            super_man_shoot_bullets:Vec::new(),
            enemy_shoot_bullets:Vec::new(),
        }
    }

    ///创建一个敌人
    pub fn add_enemy(& mut self,enemy:enemy::Enemy<'a>){
        self.enemys.push(enemy);
    }

    ///运动
    pub fn run(&mut self,e:&Event){
        //超人运动
        let bullet_option:Option<Bullet> = self.super_man.exec(e);
        match bullet_option {
            Some(bullet) => {
                self.super_man_shoot_bullets.push(bullet);
            }
            None => {
            }
        }

        //敌人运动
        let mut remove_index_list:Vec<usize> = Vec::new();
        for (index,enemy) in self.enemys.iter_mut().enumerate(){
            let bullet_opt:Option<Bullet> = enemy.run(self.super_man.coordinate);
            match bullet_opt {
                Some(bullet) => {
                    self.enemy_shoot_bullets.push(bullet);
                }
                None => {
                }
            }
            let (x,y) = enemy.coordinate;
            if x > comm::WIN_WIDTH || x < -1f64 || y > comm::WIN_HEIGHT || y < -1f64 {
                remove_index_list.push(index);
            };
        }

        //清理屏幕之外的敌人
        for index in remove_index_list{
            self.enemys.remove(index);
        }


        //删除超人失效的子弹
        let mut del_index_list = Vec::new();
        for (index,bullet) in self.super_man_shoot_bullets.iter_mut().enumerate(){
            let (x,y) = bullet.coordinate;
            if x > comm::WIN_WIDTH{
                del_index_list.push(index);
            }
            bullet.run();
        }
        for index in del_index_list{
            self.super_man_shoot_bullets.remove(index);
        }

        //删除敌人失效的子弹
        let mut del_index_list = Vec::new();
        for (index,bullet) in self.enemy_shoot_bullets.iter_mut().enumerate(){
            let (x,y) = bullet.coordinate;
            if x > comm::WIN_WIDTH{
                del_index_list.push(index);
            }
            bullet.run();
        }
        for index in del_index_list{
            self.enemy_shoot_bullets.remove(index);
        }

        //处理击中，和子弹碰撞
    }

}


///画图
impl <'a> crate::map::draw::Draw for Game<'a>{

    ///画图
    fn draw (&self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        self.super_man.draw(glyphs, c, g, device);

        //敌人
        for enemy in self.enemys.iter(){
            enemy.draw(glyphs, c, g, device);
        }

        //超人子弹
        for bullet in self.super_man_shoot_bullets.iter(){
            bullet.draw(glyphs, c, g, device);
        }
        //敌人超人子弹
        for bullet in self.enemy_shoot_bullets.iter(){
            bullet.draw(glyphs, c, g, device);
        }
    }
}