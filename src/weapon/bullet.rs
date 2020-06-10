//!子弹

use crate::config::{config, colour};
use crate::config::comm_utils;
use piston_window::*;
use std::f64::consts;

///子弹类型
pub enum BulletType{
    ///等级1,
    level1,
    ///等级二
    level2,
}

///子弹宽度百分比
pub const width_pre:f64 = 2f64 / 100f64;
///子弹高度百分比
pub const height_pre:f64 = 2f64 / 100f64;

///子弹
pub struct Bullet{
    ///坐标点
    pub coordinate: config::COORDINATE,
    ///大小
    pub win_size: config::WinSize,
    bullet_type:BulletType,
    angle:f64, //角度,弧度计算
    step_time:u32,
    step_length:f64,
}


///子弹
impl  Bullet{

    ///创建子弹,确定目标后发送子弹
    pub fn new_with_target(start: config::COORDINATE, target: config::COORDINATE, bullet_type:BulletType) -> Bullet{
        Bullet::new_with_angle(start, comm_utils::calc_angle(start, target), bullet_type)
    }
    ///创建子弹，确定角度后发送子弹
    pub fn new_with_angle(start: config::COORDINATE, angle:f64, bullet_type:BulletType) -> Bullet{
        let width:f64 = config::WIN_WIDTH * width_pre;
        let height:f64 = config::WIN_HEIGHT * height_pre;

        Bullet {
            coordinate:start,
            win_size:[width,height],
            angle:angle,
            bullet_type,
            step_time: config::BULLET_STEP_TIME,
            step_length: config::BULLET_STEP_LENGTH
        }
    }

    ///子弹运动中
    pub fn run(&mut self){
        if self.step_time <= 0u32 {
            let(x,y) = self.coordinate;
            let a = self.angle.cos() * self.step_length; //横向移动距离
            let b = self.angle.sin() * self.step_length; //纵向移动距离
            self.coordinate = (x + a,y + b);
            self.step_time = config::BULLET_STEP_TIME;
        }else{
            self.step_time = self.step_time - 1u32;
        }

    }
}


///获取坐标
impl config::CoordinateTrait for Bullet{
    fn coordinate(&self) -> (f64, f64) {
        self.coordinate
    }
}

///画敌人
impl  crate::map::draw::Draw for Bullet {
    ///画图
    fn draw(&self, glyphs: &mut Glyphs, c: Context, g: &mut G2d, device: &mut gfx_device_gl::Device) {
        let ref win_size = self.win_size;
        let ref coordinate = self.coordinate;

        unsafe {
            let mut texture: &Option<G2dTexture> = &None;
            match self.bullet_type{
                BulletType::level1 => texture = &config::BULLET_TEXTURE_LEVEL1,
                BulletType::level2 => texture = &config::BULLET_TEXTURE_LEVEL2,
                _ => {}
            }

            if let Some(image) = texture{
                // 获取图片尺寸
                let (width, height) = image.get_size();
                let(x,y) = self.coordinate;
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

