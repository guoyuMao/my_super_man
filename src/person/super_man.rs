//! 人员
use crate::config::{config, colour};
use crate::config::comm_utils;
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
    pub coordinate: config::COORDINATE,
    ///窗口大小
    pub win_size: config::WinSize,
    ///类别
    pub category: Category,
    ///状态
    pub status:Status,
    run_angle:f64, //角度
    shoot_angle:f64, //角度
    step_time:u32,
    step_length:f64,
    speed_up:bool,//加速
    fire:bool, //开火?
    fire_split_time:u16, //记录子弹发送间隔
    mouse_two_time_click:u64,//鼠标两次点击间隔时间，用于判断是否改变方向
}

///敌人
impl  SuperMan{
    ///新建敌人
    /// per:百分比，最大100
    pub fn new(per:f64,category: Category) -> SuperMan{
        assert!(per < 1f64);
        let width:f64 = config::WIN_WIDTH * per;
        let height:f64 = config::WIN_HEIGHT * per;
        SuperMan{
            coordinate:(250f64,250f64),
            win_size:[width,height],
            category,
            status:Status::Birth(config::SUPER_MAN_ALIVE_LAST_TIME),
            run_angle:0f64,
            shoot_angle:0f64,
            step_time: config::SUPER_MAN_STEP_TIME,
            step_length: config::SUPER_MAN_LENGTH,
            speed_up:false,
            fire:false,
            fire_split_time: config::SUPER_MAN_OPEN_FIRE_SPLIT_TIME,
            mouse_two_time_click:0u64
        }
    }

    ///更新超人数据
    pub fn update(&mut self, super_man:SuperMan){
        *self = super_man;
    }
    ///运动
    fn run(&mut self) {
        //运动
        if self.step_time <= 0u32 {
            let (x, y) = self.coordinate;
            let a = self.run_angle.cos() * self.step_length; //横向移动距离
            let b = self.run_angle.sin() * self.step_length; //纵向移动距离
            self.coordinate = (x + a, y + b);
            if self.speed_up{
                self.step_time = config::SUPER_MAN_STEP_TIME / 2u32;
            }else{
                self.step_time = config::SUPER_MAN_STEP_TIME;
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
        return Bullet::new_with_angle(self.coordinate,self.shoot_angle,weapon::bullet::BulletType::level2)
    }

    ///运动,返回射击的子弹
    pub fn exec(&mut self,e:&Event) -> Option<Bullet>{


        //按键盘
        if let Some(button) = e.press_args() {
            let (x,y) = self.coordinate;
            //获取键盘事件
            match button {
                //控制角度
                Button::Mouse(MouseButton::Left) => {
                    if let Some(button) = e.press_args() {

                        //按压事件
                        unsafe{
                            if let Some(mouse_x_y) = config::CURRENT_MOUSE_COORDINATE{
                                let angle = comm_utils::calc_angle(self.coordinate, mouse_x_y);
                                self.shoot_angle = angle; //射击方向
                                if self.mouse_two_time_click <= config::INTERVAL_TIME_OF_TWO_CLICK{
                                    self.run_angle = angle; //运动方向
                                }

                            }
                        };

                        //重置间隔时间
                        self.mouse_two_time_click = 0u64;
                    }
                }

                //移动位置
                Button::Keyboard(Key::Up) =>  {
                    self.coordinate = (x, y - self.step_length);
                    self.run_angle = consts::FRAC_PI_2 * 3f64;
                    self.shoot_angle = consts::FRAC_PI_2 * 3f64;
                },
                Button::Keyboard(Key::Down) => {
                    self.coordinate = (x, y + self.step_length);
                    self.run_angle = consts::FRAC_PI_2;
                    self.shoot_angle = consts::FRAC_PI_2;
                },
                Button::Keyboard(Key::Left) => {
                    self.coordinate = (x - self.step_length, y);
                    self.run_angle = consts::PI;
                    self.shoot_angle = consts::PI;
                },
                Button::Keyboard(Key::Right) => {
                    self.coordinate = (x + self.step_length, y);
                    self.run_angle = 0f64;
                    self.shoot_angle = 0f64;
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
                    self.fire_split_time = config::SUPER_MAN_OPEN_FIRE_SPLIT_TIME;
                },
                _ => {},
            }
        };

        self.mouse_two_time_click = self.mouse_two_time_click + 1;
        self.run();


        if self.fire{
            //控制子弹发射速度
            if self.fire_split_time <= 0 {
                //发射子弹
                self.fire_split_time = config::SUPER_MAN_OPEN_FIRE_SPLIT_TIME;
                return Some(self.shoot());
            }else{
                self.fire_split_time = self.fire_split_time - 1;
            }
        }
        None
    }
}

///获取坐标
impl config::CoordinateTrait for SuperMan{
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

        let mut texture: &Option<G2dTexture> = &None;
        unsafe {
            match self.category {
                Category::type1 => texture = &config::SUPER_MAN_TEXTURE1,
                Category::type2 => texture = &config::SUPER_MAN_TEXTURE2,
                _ => {}
            }
        }

        if let Some(image) = texture{
            // 获取图片尺寸
            let (width, height) = image.get_size();
            let (x,y) = self.coordinate;//图片中心位置
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