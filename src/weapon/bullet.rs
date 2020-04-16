//!子弹

use crate::config::{comm,colour};
use piston_window::*;

///子弹宽度百分比
pub const width_pre:f64 = 2f64 / 100f64;
///子弹高度百分比
pub const height_pre:f64 = 2f64 / 100f64;

///子弹
pub struct Bullet{
    ///坐标点
    pub coordinate:comm::COORDINATE,
    win_size:comm::WIN_SIZE,
    angle:f64, //角度,弧度计算
    speed:f64,
}


///子弹
impl  Bullet{

    ///创建子弹,确定目标后发送子弹
    pub fn new_with_target(start:comm::COORDINATE,target:comm::COORDINATE) -> Bullet{
        let (x,y) = start;
        let (x1,y1) = target;
        let y1 = -y1; //游戏坐标系与实际坐标系上下相反
        let a = x1 - x; //a
        let b = y1 - y; //b
        println!("a:{},b:{}", a, b);
        let angle = (a/b).atan();
        Bullet::new_with_angle(start,angle)
    }
    ///创建子弹，确定角度后发送子弹
    pub fn new_with_angle(start:comm::COORDINATE,angle:f64) -> Bullet{
        let width:f64 = comm::WIN_WIDTH * width_pre;
        let height:f64 = comm::WIN_HEIGHT * height_pre;

        Bullet {
            coordinate:start,
            win_size:[width,height],
            angle:angle,
            speed:comm::BULLET_SPEED,
        }
    }

    ///子弹运动中
    pub fn run(&mut self){
        let(x,y) = self.coordinate;
        let a = self.angle.cos() * comm::BULLET_SPEED; //横向移动距离
        let b = self.angle.sin() * comm::BULLET_SPEED; //纵向移动距离
        self.coordinate = (x + a,y + b);
    }
}

///画敌人
impl  crate::map::draw::Draw for Bullet {
    ///画图
    fn draw(&self, glyphs: &mut Glyphs, c: Context, g: &mut G2d, device: &mut gfx_device_gl::Device) {
        let ref win_size = self.win_size;
        let ref coordinate = self.coordinate;

        unsafe {
            match &comm::BULLET_TEXTURE {
                Some(image) => {
                    // 获取图片尺寸
                    let (width, height) = image.get_size();
                    let(x,y) = self.coordinate;
                    Image::new().draw(
                        image,
                        &c.draw_state,
                        c.transform
                            .trans(x,y) //相对位置
                            .scale(self.win_size[0] / width as f64, self.win_size[1] / height as f64),    //缩放
                        g);
                }
                None => {
                    panic!("子弹图片加载失败")
                }
            }
        }
    }
}

