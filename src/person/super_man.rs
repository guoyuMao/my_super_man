//! 人员
use crate::config::{comm,colour};
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;
use std::f64::consts;

///超人
pub struct Super_man<'a>{
    ///坐标点
    pub coordinate:comm::COORDINATE,
    win_size:comm::WIN_SIZE,
    ///image
    pub image: &'a G2dTexture,
    angle:f64, //角度
    speed:f64, //速度
    i:u8, //记录子弹发送间隔
}

///敌人
impl <'a> Super_man<'a>{
    ///新建敌人
    /// per:百分比，最大100
    pub fn new(per:f64,image:&'a G2dTexture) -> Super_man{
        assert!(per < 1f64);
        let width:f64 = comm::WIN_WIDTH * per;
        let height:f64 = comm::WIN_HEIGHT * per;
        Super_man{
            coordinate:(50f64,50f64),
            win_size:[width,height],
            image:image,
            angle:0f64,
            speed:comm::BASE_SPEED,
            i:0,
        }
    }

    ///发送子弹
    fn shoot(& mut self) -> Bullet{
        Bullet::new_with_angle(self.coordinate,self.angle)
    }

    ///运动,返回射击的子弹
    pub fn exec(&mut self,e:&Event) -> Option<Bullet>{
        //按键盘
        if let Some(button) = e.press_args() {
            let (x,y) = self.coordinate;
            //获取键盘事件
            match button {
                //移动位置
                Button::Keyboard(Key::Up) =>  {
                    self.coordinate = (x, y - self.speed);
                    self.angle = consts::FRAC_PI_2;
                },
                Button::Keyboard(Key::Down) => {
                    self.coordinate = (x, y + self.speed);
                    self.angle = consts::FRAC_PI_3 * 4f64;
                },
                Button::Keyboard(Key::Left) => {
                    self.coordinate = (x - self.speed, y);
                    self.angle = consts::FRAC_1_PI;
                },
                Button::Keyboard(Key::Right) => {
                    self.coordinate = (x + self.speed, y);
                    self.angle = 0f64;
                }
                Button::Keyboard(Key::Space) => {
                    self.speed = comm::ACCELERATION;    //加速度
                }
                Button::Keyboard(Key::B) => {
                     //发送子弹
                    return Some(self.shoot());
                }
                _ => {},
            }
        }

        //释放键盘
        if let Some(button) = e.release_args(){
            match button {
                //释放速度
                Button::Keyboard(Key::Space) => self.speed = comm::BASE_SPEED,
                _ => {},
            }
        };

        None
    }
}

///画敌人
impl <'a> crate::map::draw::Draw for Super_man<'a>{

    ///画图
    fn draw (& self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        let ref win_size = self.win_size;
        let (x,y) = self.coordinate;

        // 获取图片尺寸
        let (width, height) = self.image.get_size();
        Image::new().draw(
            self.image,
            &c.draw_state,
            c.transform
                .trans(x, y) //相对位置
                .scale(self.win_size[0] / width as f64, self.win_size[1] / height as f64),    //缩放
            g);
    }
}