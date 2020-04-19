//!游戏要素
use super::person::{enemy,super_man};
use super::weapon::{bullet};
use crate::config::comm;
use piston_window::*;
use crate::weapon::bullet::Bullet;
use crate::weapon::explode::{Explode, ExplodeType};


///游戏要素
pub struct Game<'a>{
    ///穿件敌人的间隔事件
    create_enemy_split_time:u32,
    ///超人
    pub super_man:super_man::Super_man<'a>,
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
impl<'a> Game<'a>{

    ///new Game
    pub fn new(super_man:super_man::Super_man<'a>)-> Game<'a>{
        Game{
            create_enemy_split_time:comm::CREATE_ENEMY_SPLIT_TIME,
            super_man,
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

        //创建敌人
        if self.create_enemy_split_time <= 0{
            let mut enemy = enemy::Enemy::new(enemy::EnemyCategory::type2 ,self.super_man.coordinate);
            self.add_enemy(enemy);
            self.create_enemy_split_time = comm::CREATE_ENEMY_SPLIT_TIME;
        }else{
            self.create_enemy_split_time = self.create_enemy_split_time - 1;
        }

        //获取鼠标坐标
        if let Some(mouse_x_y) = e.mouse_cursor_args(){
            unsafe{
            comm::CURRENT_MOUSE_COORDINATE = Some((mouse_x_y[0],mouse_x_y[1]));
            }
        }


        //超人运动
        let bullet_option:Option<Bullet> = self.super_man.exec(e);
        if let Some(bullet) = bullet_option{
            self.super_man_shoot_bullets.push(bullet);
        }

        //敌人运动
        for enemy in self.enemys.iter_mut(){
            let bullet_option:Option<Bullet> = enemy.run(self.super_man.coordinate);
            if let Some(bullet) = bullet_option{
                self.enemy_shoot_bullets.push(bullet);
            }
        }

        //删除超人失效的子弹
        let mut del_coordinate_list = Vec::new();
        for bullet in self.super_man_shoot_bullets.iter_mut(){
            let (x,y) = bullet.coordinate;
            if x > comm::WIN_WIDTH || x <= 0f64 || y > comm::WIN_HEIGHT || y <= 0f64{
                del_coordinate_list.push(bullet.coordinate);
            }
            bullet.run();
        }
        self.super_man_shoot_bullets.retain(|x|{
           return !del_coordinate_list.contains(&x.coordinate)
        });

        //删除敌人失效的子弹
        let mut del_coordinate_list = Vec::new();
        for bullet in self.enemy_shoot_bullets.iter_mut(){
            let (x,y) = bullet.coordinate;
            if x > comm::WIN_WIDTH || x <= 0f64 || y > comm::WIN_HEIGHT || y <= 0f64{
                del_coordinate_list.push(bullet.coordinate);
            }
            bullet.run();
        }
        self.enemy_shoot_bullets.retain(|x|{
            return !del_coordinate_list.contains(&x.coordinate);
        });

        let mut enemy_del_coordinate_list:Vec<comm::COORDINATE> = Vec::new();
        let mut bullet_del_coordinate_list:Vec<comm::COORDINATE> = Vec::new();
        //计算死亡的敌人
        for  enemy in self.enemys.iter() {
            let (e_x1, e_y1) = enemy.coordinate;
            let e_x2 = e_x1 + enemy.win_size[0];
            let e_y2 = e_y1 + enemy.win_size[1];
            //遍历敌人
            for bullet in self.super_man_shoot_bullets.iter() {
                let (m_x1,m_y1) = bullet.coordinate;
                let m_x2 = m_x1 + bullet.win_size[0];
                let m_y2:f64 = m_y1 + bullet.win_size[1];
                if e_x2 > m_x1 && e_x1 < m_x2{
                    //x重叠
                    if e_y2 > m_y1 && e_y1 < m_y2{
                        //y重叠
                        enemy_del_coordinate_list.push(enemy.coordinate);
                        bullet_del_coordinate_list.push(bullet.coordinate);

                        let mx = (m_x1 + m_x2) / 2f64;
                        let my = (m_y1 + m_y2) / 2f64;
                        let ex = (e_x1 + e_x2) / 2f64;
                        let ey = (e_y1 + e_y2) / 2f64;
                        //爆炸点坐标
                        let coordinate = ((mx + ex)/2f64,(my+ey) / 2f64);
                        let explode = Explode::new(coordinate,ExplodeType::type1);
                        self.explodes.push(explode);

                    }
                }
            }
        }

        self.enemys.retain(|x|{
            return !enemy_del_coordinate_list.contains(&x.coordinate);
        });
        self.super_man_shoot_bullets.retain(|x|{
            return !bullet_del_coordinate_list.contains(&x.coordinate);
        });

        //删除敌人失效的子弹
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

        //画爆炸点
        for explode in self.explodes.iter(){
            explode.draw(glyphs, c, g, device);
        }

    }
}