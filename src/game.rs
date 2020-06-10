//!游戏要素

use super::person::{enemy,super_man};
use super::weapon::{bullet};
use crate::config::config;
use crate::config::comm_utils;
use piston_window::*;
use crate::weapon::bullet::Bullet;
use crate::weapon::explode::{Explode, ExplodeType};

///游戏模式
pub enum Mode{
    ///单人模式
    Single,
    ///双人模式
    Double,
}

///游戏要素
pub struct Game{
    ///游戏模式
    pub mode:Option<Mode>,
    ///穿件敌人的间隔事件
    create_enemy_split_time:u32,
    ///超人
    pub super_mans:Vec<super_man::SuperMan>,
    ///敌人
    enemys:Vec<enemy::Enemy>,
    ///超人发射的子弹
    super_man_shoot_bullets:Vec<bullet::Bullet>,
    ///敌人发射的子弹
    enemy_shoot_bullets:Vec<bullet::Bullet>,
    ///记录爆炸点
    explodes:Vec<Explode>,

}

///game operator
impl Game{

    ///new Game
    pub fn new(super_mans:Vec<super_man::SuperMan>)-> Game{
        Game{
            mode:None,
            create_enemy_split_time: config::CREATE_ENEMY_SPLIT_TIME,
            super_mans,
            enemys:Vec::new(),
            super_man_shoot_bullets:Vec::new(),
            enemy_shoot_bullets:Vec::new(),
            explodes:Vec::new(),
        }
    }

    ///创建一个敌人
    fn add_enemy(& mut self,enemy:enemy::Enemy){
        self.enemys.push(enemy);
    }

    ///运动
    pub fn run(&mut self,e:&Event){
        match self.mode{
            None =>{
                //按键盘
                if let Some(button) = e.press_args() {
                    match button {
                        //移动位置
                        Button::Keyboard(Key::D1) =>  {
                            self.mode = Some(Mode::Single)
                        },
                        Button::Keyboard(Key::D2) => {
                            self.mode = Some(Mode::Double)
                        },
                        _ => {},
                    }
                }
            }
            _ => {
                //定时创建敌人
                if self.create_enemy_split_time <= 0{
                    let mut enemy = enemy::Enemy::new(enemy::EnemyCategory::type2 ,&self.super_mans);
                    self.add_enemy(enemy);
                    self.create_enemy_split_time = config::CREATE_ENEMY_SPLIT_TIME;
                }

                self.create_enemy_split_time = self.create_enemy_split_time - 1;


                //获取鼠标坐标
                if let Some(mouse_x_y) = e.mouse_cursor_args(){
                    unsafe{
                        config::CURRENT_MOUSE_COORDINATE = Some((mouse_x_y[0], mouse_x_y[1]));
                    }
                }


                //-------------------------------------------运动效果-----------------------------------------
                //超人运动
                for super_man in self.super_mans.iter_mut(){
                    let bullet_option:Option<Bullet> = super_man.exec(e);
                    if let Some(bullet) = bullet_option{
                        self.super_man_shoot_bullets.push(bullet);
                    }
                }

                //敌人运动
                for enemy in self.enemys.iter_mut(){
                    let bullet_option:Option<Bullet> = enemy.run(&self.super_mans);
                    if let Some(bullet) = bullet_option{
                        self.enemy_shoot_bullets.push(bullet);
                    }
                }

                //子弹运动
                self.super_man_shoot_bullets.iter_mut().for_each(|x|{
                    x.run();
                });
                self.enemy_shoot_bullets.iter_mut().for_each(|x|{
                    x.run();
                });


                //---------------------清楚失效数据--------------------------------
                //清楚失效子弹
                config::clean_coordinate(&mut self.super_man_shoot_bullets);
                config::clean_coordinate(&mut self.enemy_shoot_bullets);
                //删除爆炸效果
                let mut del_coordinate_list = Vec::new();
                for bullet in self.explodes.iter_mut(){
                    bullet.exec();
                    if bullet.exist_time <= 0{
                        del_coordinate_list.push(bullet.coordinate);
                    }
                }
                self.explodes.retain(|x| {
                    return !del_coordinate_list.contains(&x.coordinate);
                });

                //--------------------------------------------中弹----------------------------------
                let mut enemy_del_coordinate_list:Vec<config::COORDINATE> = Vec::new();
                let mut bullet_del_coordinate_list:Vec<config::COORDINATE> = Vec::new();
                //计算死亡的敌人
                for  enemy in self.enemys.iter() {
                    //遍历敌人
                    for bullet in self.super_man_shoot_bullets.iter() {

                        if comm_utils::is_collide(enemy.coordinate, enemy.win_size, bullet.coordinate, bullet.win_size){
                            //发生碰撞
                            enemy_del_coordinate_list.push(enemy.coordinate);
                            bullet_del_coordinate_list.push(bullet.coordinate);

                            //爆炸点坐标
                            let coordinate = comm_utils::calc_core(enemy.coordinate,bullet.coordinate);
                            let explode = Explode::new(coordinate,ExplodeType::type1);
                            self.explodes.push(explode);
                        }
                    }
                }
                //清楚爆炸点的敌人
                self.enemys.retain(|x|{
                    return !enemy_del_coordinate_list.contains(&x.coordinate);
                });
                //清楚爆炸点的子弹
                self.super_man_shoot_bullets.retain(|x|{
                    return !bullet_del_coordinate_list.contains(&x.coordinate);
                });

                //超人坐标被杀死
                let mut del_coordinate_list:Vec<config::COORDINATE> = Vec::new();
                for super_man in self.super_mans.iter(){
                    if let super_man::Status::Alive = super_man.status{
                        //超人还活着
                        let mut bullet_del_coordinate_list:Vec<config::COORDINATE> = Vec::new();
                        for bullet in self.enemy_shoot_bullets.iter(){
                            if comm_utils::is_collide(super_man.coordinate,super_man.win_size,bullet.coordinate,bullet.win_size){
                                bullet_del_coordinate_list.push(bullet.coordinate);

                                //爆炸点坐标
                                let coordinate = comm_utils::calc_core(super_man.coordinate, bullet.coordinate);
                                let explode = Explode::new(coordinate,ExplodeType::type2);
                                self.explodes.push(explode);//爆炸效果
                                del_coordinate_list.push(super_man.coordinate);
                            }
                        }
                    }
                }

                //清楚爆炸点的敌人
                self.super_mans.retain(|x|{
                    return !del_coordinate_list.contains(&x.coordinate);
                });
                for _ in del_coordinate_list{
                    //创建新的超人
                    self.super_mans.push(super_man::SuperMan::new(config::PERSON_SIZE, super_man::Category::type2));
                }
            }
        }
    }
}


///画图
impl  crate::map::draw::Draw for Game{

    ///画图
    fn draw (&self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        match self.mode{
            None => {
                //选择游戏模式
                let text_mode = format!("please choice mode: 1:Single 2:Double");
                let transform = c.transform.trans(
                    10f64,
                    30f64,
                );

                Text::new_color(config::BLACK, config::FONT_SIZE).draw(
                    &text_mode[..],
                    glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            }
            _ => {
                //继续游戏
                for super_man in self.super_mans.iter(){
                    super_man.draw(glyphs, c, g, device);
                }

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

                //画爆炸点
                for explode in self.explodes.iter(){
                    explode.draw(glyphs, c, g, device);
                }



            }
        }
    }
}