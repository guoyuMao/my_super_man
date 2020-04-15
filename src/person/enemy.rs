//! 人员
use crate::config::{comm,colour};
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;

///敌人
pub struct Enemy<'a>{
    coordinate:comm::COORDINATE,
    win_size:comm::WIN_SIZE,
    colour:colour::Colour,
    ///image
    pub image: &'a G2dTexture,
    i:u8, //记录子弹发送间隔
}

///敌人
impl <'a> Enemy<'a>{
    ///新建敌人
    /// per:百分比，最大100
    pub fn new(per:u8,image:&'a G2dTexture) -> Enemy{
        assert!(per < 100);
        let width:f64 = comm::WIN_WIDTH * per as f64 / 100f64;
        let height:f64 = comm::WIN_HEIGHT * per as f64 / 100f64;
        Enemy{
            coordinate:[20f64,20f64],
            win_size:[width,height],
            colour:colour::RED,
            image:image,
            i:0,
        }
    }

    ///发送子弹
    pub fn shoot(& mut self) -> Bullet{
        Bullet::new(self.coordinate)
    }

    ///运动
    pub fn run(&mut self) -> Option<Bullet>{
        self.coordinate[0] = self.coordinate[0] + 0.1f64;
        self.i = self.i + 1;
        if self.i > 100 {
            self.i = 0;
            return Some(self.shoot())
        }else{
            return None
        }
    }
}

///画敌人
impl <'a> crate::map::draw::Draw for Enemy<'a>{

    ///画图
    fn draw (& self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        let ref win_size = self.win_size;
        let ref coordinate = self.coordinate;

        // 获取图片尺寸
        let (width, height) = self.image.get_size();
        Image::new().draw(
            self.image,
            &c.draw_state,
            c.transform
                .trans(self.coordinate[0], self.coordinate[1]) //相对位置
                .scale(self.win_size[0] / width as f64, self.win_size[1] / height as f64),    //缩放
            g);
    }
}