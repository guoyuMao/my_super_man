//! 人员
extern crate rand;
use rand::{thread_rng, Rng};
use crate::config::{config, colour};
use crate::config::comm_utils;
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;
use std::cmp::Ordering;

///敌人类型
pub enum EnemyCategory{
    ///等级1,
    type1,
    ///等级二
    type2,
}

///敌人
pub struct Enemy{
    ///坐标点
    pub coordinate: config::COORDINATE,
    ///尺寸
    pub win_size: config::WinSize,
    colour:colour::Colour,
    ///运动角度
    pub angle:f64, //角度,弧度计算
    category:EnemyCategory,
    step_time:u32,
    step_length:f64,
    fire_split_time:u16, //记录子弹发送间隔
}

///敌人
impl Enemy{
    ///新建敌人
    pub fn new(category:EnemyCategory,super_mans:&Vec<super::super_man::SuperMan>) -> Enemy{

        let mut my_coordinate:(f64,f64);

        let mut rng =rand::thread_rng();
        let a:u16 = rng.gen();

        //在上下产生敌人
        'outer: loop {
            let m:u32 = rng.gen();
            let n:u32 = m % config::WIN_WIDTH as u32;
            let mut coordinate:(f64,f64) = (0f64, n as f64);
            if a % 2 == 0 {
                coordinate = (n as f64, 0f64);
            }

            if n >= 0 {
                for super_name in super_mans {
                    if config::calc_distance(coordinate, super_name.coordinate) > config::CREATE_ENEMY_DISTANT {
                        //找到生成敌人的坐标点
                        my_coordinate = coordinate;
                        break 'outer;
                    }
                }
            }
        }

        let mut super_man = &super_mans[0];
        if super_mans.len() >=2{
            //获取距离最近的超人作为攻击目标
            super_man = super_mans.iter().max_by(|a,b|{
                if config::calc_distance(my_coordinate, a.coordinate) > config::calc_distance(my_coordinate, b.coordinate){
                    Ordering::Greater
                }else{
                    Ordering::Less
                }
            }).unwrap();
        }

        let width:f64 = config::WIN_WIDTH * config::PERSON_SIZE;
        let height:f64 = config::WIN_HEIGHT * config::PERSON_SIZE;
        Enemy{
            coordinate:my_coordinate,
            win_size:[width,height],
            colour:colour::RED,
            angle: comm_utils::calc_angle(my_coordinate, super_man.coordinate),
            category:category,
            step_time: config::ENEMY_STEP_TIME,
            step_length: config::ENEMY_STEP_LENGTH,
            fire_split_time: config::ENEMY_OPEN_FIRE_SPLIT_TIME,
        }
    }

    ///发送子弹
    fn shoot(& mut self, target: config::COORDINATE) -> Bullet{
        Bullet::new_with_target(self.coordinate,target,weapon::bullet::BulletType::level1)
    }

    ///运动,返回射击的子弹
    pub fn run(&mut self,super_mans:&Vec<super::super_man::SuperMan>) -> Option<Bullet>{
        let mut super_man = &super_mans[0];
        if super_mans.len() >=2{
            //获取距离最近的超人作为攻击目标
            super_man = super_mans.iter().max_by(|a,b|{
                if config::calc_distance(self.coordinate, a.coordinate) > config::calc_distance(self.coordinate, b.coordinate){
                    Ordering::Greater
                }else{
                    Ordering::Less
                }
            }).unwrap();
        }
        let target = super_man.coordinate;

        //运动
        if self.step_time <= 0u32{
            let(x,y) = self.coordinate;
            let a = self.angle.cos() * self.step_length; //横向移动距离
            let b = self.angle.sin() * self.step_length; //纵向移动距离
            self.coordinate = (x + a,y + b);
            self.step_time  = config::ENEMY_STEP_TIME;
        }else{
            self.step_time = self.step_time - 1u32;
        }


        //从新运算目标
        self.angle = comm_utils::calc_angle(self.coordinate, target);

        //发射子弹
        if self.fire_split_time <= 0 {
            //间隔100次射击一次
            self.fire_split_time = config::ENEMY_OPEN_FIRE_SPLIT_TIME;
            return Some(self.shoot(target))
        }else{
            self.fire_split_time = self.fire_split_time - 1;
            return None
        }
    }
}

///获取坐标
impl config::CoordinateTrait for Enemy{
    fn coordinate(&self) -> (f64, f64) {
        self.coordinate
    }
}

///画敌人
impl  crate::map::draw::Draw for Enemy{

    ///画图
    fn draw (& self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        let ref win_size = self.win_size;
        let ref coordinate = self.coordinate;

        unsafe {
            let mut texture: &Option<G2dTexture> = &None;
            match self.category{
                EnemyCategory::type1 => texture = &config::ENEMY_TEXTURE1,
                EnemyCategory::type2 => texture = &config::ENEMY_TEXTURE2,
                _ => {}
            }

            if let Some(image) = texture{
                // 获取图片尺寸
                let (width, height) = image.get_size();
                let (x,y) = self.coordinate;
                let x1 = self.win_size[0] / width as f64;
                let y1 = self.win_size[1] / height as f64;
                Image::new().draw(
                    image,
                    &c.draw_state,
                    c.transform
                        .trans(x-(x1/2f64), y-(y1/2f64)) //相对位置
                        .scale(x1, y1),    //缩放
                    g);
            }
        }
    }
}