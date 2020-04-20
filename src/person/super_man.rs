//! 人员
use crate::config::{comm,colour};
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;
use std::f64::consts;


///敌人类型
pub enum Category{
    ///等级1,
    type1,
    ///等级二
    type2,
}

///状态
pub enum Status{
    ///出生
    Birth(u32),
    ///活着
    Alive,
}

///超人
pub struct SuperMan{
    ///坐标点
    pub coordinate:comm::COORDINATE,
    ///窗口大小
    pub win_size:comm::WIN_SIZE,
    ///类别
    pub category: Category,
    ///状态
    pub status:Status,
    angle:f64, //角度
    step_time:u32,
    step_length:f64,
    speed_up:bool,//加速
    fire:bool, //开火?
    fire_split_time:u16, //记录子弹发送间隔
}

///敌人
impl  SuperMan{
    ///新建敌人
    /// per:百分比，最大100
    pub fn new(per:f64,category: Category) -> SuperMan{
        assert!(per < 1f64);
        let width:f64 = comm::WIN_WIDTH * per;
        let height:f64 = comm::WIN_HEIGHT * per;
        SuperMan{
            coordinate:(250f64,250f64),
            win_size:[width,height],
            category,
            status:Status::Birth(comm::SUPER_MAN_ALIVE_LAST_TIME),
            angle:0f64,
            step_time:comm::SUPER_MAN_STEP_TIME,
            step_length:comm::SUPER_MAN_LENGTH,
            speed_up:false,
            fire:false,
            fire_split_time:comm::SUPER_MAN_OPEN_FIRE_SPLIT_TIME,
        }
    }

    ///运动
    fn run(&mut self) {
        //运动
        if self.step_time <= 0u32 {
            let (x, y) = self.coordinate;
            let a = self.angle.cos() * self.step_length; //横向移动距离
            let b = self.angle.sin() * self.step_length; //纵向移动距离
            self.coordinate = (x + a, y + b);
            if self.speed_up{
                self.step_time = comm::SUPER_MAN_STEP_TIME / 2u32;
            }else{
                self.step_time = comm::SUPER_MAN_STEP_TIME;
            }
        } else {
            self.step_time = self.step_time - 1u32;
        }

        if let Status::Birth(t) = self.status{
            //修改成长状态
            if t <= 0u32{
                self.status = Status::Alive;
            }else{
                self.status = Status::Birth(t - 1);
            }
        }
    }

        ///发送子弹
    fn shoot(& mut self) -> Bullet{
        return Bullet::new_with_angle(self.coordinate,self.angle,weapon::bullet::BulletType::level2)
    }

    ///运动,返回射击的子弹
    pub fn exec(&mut self,e:&Event) -> Option<Bullet>{
        self.run();

        //按键盘
        if let Some(button) = e.press_args() {
            let (x,y) = self.coordinate;
            //获取键盘事件
            match button {
                //控制角度
                Button::Mouse(MouseButton::Left) => {
                    //获取鼠标坐标
                    unsafe{
                            if let Some(button) = e.press_args() {
                                //按压事件
                                if let Some(mouse_x_y) = comm::CURRENT_MOUSE_COORDINATE{
                                    let angle = comm::calc_angle(self.coordinate, mouse_x_y);
                                    self.angle = angle;
                                }
                            };
                    }
                }

                //移动位置
                Button::Keyboard(Key::Up) =>  {
                    self.coordinate = (x, y - self.step_length);
                    self.angle = consts::FRAC_PI_2 * 3f64;
                },
                Button::Keyboard(Key::Down) => {
                    self.coordinate = (x, y + self.step_length);
                    self.angle = consts::FRAC_PI_2;
                },
                Button::Keyboard(Key::Left) => {
                    self.coordinate = (x - self.step_length, y);
                    self.angle = consts::PI;
                },
                Button::Keyboard(Key::Right) => {
                    self.coordinate = (x + self.step_length, y);
                    self.angle = 0f64;
                }
                Button::Keyboard(Key::B) => {
                    //发送子弹
                    self.fire = true;
                }
                Button::Keyboard(Key::Space) => {
                    self.speed_up = true;    //加速度
                }
                _ => {},
            }
        }

        //释放键盘
        if let Some(button) = e.release_args(){
            match button {
                //释放速度
                Button::Keyboard(Key::Space) => self.speed_up = false,
                //停止射击
                Button::Keyboard(Key::B) => {
                    self.fire = false;
                    self.fire_split_time = comm::SUPER_MAN_OPEN_FIRE_SPLIT_TIME;
                },
                _ => {},
            }
        };

        if self.fire{
            //控制子弹发射速度
            if self.fire_split_time <= 0 {
                //发射子弹
                self.fire_split_time = comm::SUPER_MAN_OPEN_FIRE_SPLIT_TIME;
                return Some(self.shoot());
            }else{
                self.fire_split_time = self.fire_split_time - 1;
            }
        }
        None
    }
}

///获取坐标
impl comm::COORDINATE_TRAIT for SuperMan{
    fn coordinate(&self) -> (f64, f64) {
        self.coordinate
    }
}

///画敌人
impl  crate::map::draw::Draw for SuperMan{

    ///画图
    fn draw (& self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        let ref win_size = self.win_size;
        let (x,y) = self.coordinate;

        unsafe {
            let mut texture: &Option<G2dTexture> = &None;
            match self.category {
                Category::type1 => texture = &comm::SUPER_MAN_TEXTURE1,
                Category::type2 => texture = &comm::SUPER_MAN_TEXTURE2,
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