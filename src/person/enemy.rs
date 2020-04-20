//! 人员
use crate::config::{comm,colour};
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;

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
    pub coordinate:comm::COORDINATE,
    ///尺寸
    pub win_size:comm::WIN_SIZE,
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
    pub fn new(category:EnemyCategory,target:comm::COORDINATE) -> Enemy{
        let coordinate = (200f64,200f64);
        let width:f64 = comm::WIN_WIDTH * comm::PERSON_SIZE;
        let height:f64 = comm::WIN_HEIGHT * comm::PERSON_SIZE;
        Enemy{
            coordinate:coordinate,
            win_size:[width,height],
            colour:colour::RED,
            angle:comm::calc_angle(coordinate,target),
            category:category,
            step_time:comm::ENEMY_STEP_TIME,
            step_length:comm::ENEMY_STEP_LENGTH,
            fire_split_time:comm::ENEMY_OPEN_FIRE_SPLIT_TIME,
        }
    }

    ///发送子弹
    fn shoot(& mut self,target:comm::COORDINATE) -> Bullet{
        Bullet::new_with_target(self.coordinate,target,weapon::bullet::BulletType::level1)
    }

    ///运动,返回射击的子弹
    pub fn run(&mut self,target:comm::COORDINATE) -> Option<Bullet>{
        //运动
        if self.step_time <= 0u32{
            let(x,y) = self.coordinate;
            let a = self.angle.cos() * self.step_length; //横向移动距离
            let b = self.angle.sin() * self.step_length; //纵向移动距离
            self.coordinate = (x + a,y + b);
            self.step_time  = comm::ENEMY_STEP_TIME;
        }else{
            self.step_time = self.step_time - 1u32;
        }


        //从新运算目标
        self.angle = comm::calc_angle(self.coordinate, target);

        //发射子弹
        if self.fire_split_time <= 0 {
            //间隔100次射击一次
            self.fire_split_time = comm::ENEMY_OPEN_FIRE_SPLIT_TIME;
            return Some(self.shoot(target))
        }else{
            self.fire_split_time = self.fire_split_time - 1;
            return None
        }
    }
}

///获取坐标
impl comm::COORDINATE_TRAIT for Enemy{
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
                EnemyCategory::type1 => texture = &comm::ENEMY_TEXTURE1,
                EnemyCategory::type2 => texture = &comm::ENEMY_TEXTURE2,
                _ => {}
            }

            if let Some(image) = texture{
                // 获取图片尺寸
                let (width, height) = image.get_size();
                let (x,y) = self.coordinate;
                Image::new().draw(
                    image,
                    &c.draw_state,
                    c.transform
                        .trans(x, y) //相对位置
                        .scale(self.win_size[0] / width as f64, self.win_size[1] / height as f64),    //缩放
                    g);
            }
        }
    }
}