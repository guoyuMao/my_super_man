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
}


///子弹
impl  Bullet{

    ///创建一格子弹
    pub fn new(coordinate:comm::COORDINATE) -> Bullet{
        let width:f64 = comm::WIN_WIDTH * width_pre;
        let height:f64 = comm::WIN_HEIGHT * height_pre;

        Bullet {
            coordinate:coordinate,
            win_size:[width,height],
        }
    }

    ///子弹运动中
    pub fn run(&mut self){
        self.coordinate[0] = self.coordinate[0] + 1f64;
    }
}

///画敌人
impl  crate::map::draw::Draw for Bullet {
    ///画图
    fn draw(&self, glyphs: &mut Glyphs, c: Context, g: &mut G2d, device: &mut gfx_device_gl::Device) {
        let ref win_size = self.win_size;
        let ref coordinate = self.coordinate;
        //画title区
        let pos: [f64; 4] = [
            coordinate[0],
            coordinate[1],
            win_size[0],
            win_size[1],
        ];

        unsafe {
            match &comm::BULLET_TEXTURE {
                Some(image) => {
                    // 获取图片尺寸
                    let (width, height) = image.get_size();
                    Image::new().draw(
                        image,
                        &c.draw_state,
                        c.transform
                            .trans(self.coordinate[0], self.coordinate[1]) //相对位置
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

