//! 人员
use crate::config::{comm,colour};
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;

///敌人
pub struct Enemy<'a>{
    ///坐标点
    pub coordinate:comm::COORDINATE,
    win_size:comm::WIN_SIZE,
    colour:colour::Colour,
    ///image
    pub image: &'a G2dTexture,
    speed:f64,
    i:u16, //记录子弹发送间隔
}

///敌人
impl <'a> Enemy<'a>{
    ///新建敌人
    /// per:百分比，最大100
    pub fn new(per:f64,image:&'a G2dTexture) -> Enemy{
        assert!(per < 1f64);
        let width:f64 = comm::WIN_WIDTH * per;
        let height:f64 = comm::WIN_HEIGHT * per;
        Enemy{
            coordinate:(200f64,200f64),
            win_size:[width,height],
            colour:colour::RED,
            image:image,
            speed:comm::BASE_SPEED,
            i:0,
        }
    }

    ///发送子弹
    fn shoot(& mut self,target:comm::COORDINATE) -> Bullet{
        Bullet::new_with_target(self.coordinate,target,weapon::bullet::BulletType::level1)
    }

    ///运动,返回射击的子弹
    pub fn run(&mut self,target:comm::COORDINATE) -> Option<Bullet>{
        let (x,y) = self.coordinate;
        self.coordinate = (x + self.speed * 0.01,y);
        self.i = self.i + 1;
        if self.i > 300 {
            //间隔100次射击一次
            println!("发射子弹");
            self.i = 0;
            return Some(self.shoot(target))
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
        let (x,y) = self.coordinate;
        Image::new().draw(
            self.image,
            &c.draw_state,
            c.transform
                .trans(x, y) //相对位置
                .scale(self.win_size[0] / width as f64, self.win_size[1] / height as f64),    //缩放
            g);
    }
}