//! config-comm
use std::option::Option;
use piston_window::G2dTexture;
use std::f64::consts;

///定义颜色类型
pub type Colour = [f32;4];
///黑色
pub const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
///字体大小
pub const FONT_SIZE1:u32 = 32u32;
///字体大小
pub const FONT_SIZE10:u32 = 15u32;

///数据库接地址
pub const DATABASE_URL:&str ="mysql://my_super_man_user:my_super_man_password@127.0.0.1:3306/my_super_man_db?pool_min=10&pool_max=100&conn_ttl=10";

///人物所占屏幕百分比
pub const PERSON_SIZE:f64 = 0.05f64;

///字体大小
pub const FONT_SIZE:u32 = 32u32;

///坐标
pub type COORDINATE = (f64,f64);
///坐标位置
pub trait CoordinateTrait{
    ///获取坐标点
    fn coordinate(&self) -> COORDINATE;
}

///窗口宽
pub const WIN_WIDTH:f64 = 1000f64;
///窗口高
pub const WIN_HEIGHT:f64 = 500f64;

///窗口大小
pub type WinSize = [f64;2];

///超人1
pub static mut SUPER_MAN_TEXTURE1:Option<G2dTexture> = None;
///超人2
pub static mut SUPER_MAN_TEXTURE2:Option<G2dTexture> = None;
///子弹图片1
pub static mut BULLET_TEXTURE_LEVEL1:Option<G2dTexture> = None;
///子弹图片2
pub static mut BULLET_TEXTURE_LEVEL2:Option<G2dTexture> = None;
///敌人1
pub static mut ENEMY_TEXTURE1:Option<G2dTexture> = None;
///敌人2
pub static mut ENEMY_TEXTURE2:Option<G2dTexture> = None;
///爆炸1
pub static mut EXPLODE_TEXTURE1:Option<G2dTexture> = None;
///爆炸2
pub static mut EXPLODE_TEXTURE2:Option<G2dTexture> = None;

///鼠标连续两次点击的判别时间
pub const INTERVAL_TIME_OF_TWO_CLICK:u64 = 200u64;


///鼠标位置
pub static mut CURRENT_MOUSE_COORDINATE:Option<COORDINATE> = None;

///超人出生状态维持时间
pub const SUPER_MAN_ALIVE_LAST_TIME:u32 = 6000u32;
///生产敌人的间隔时间
pub const CREATE_ENEMY_SPLIT_TIME:u32 = 2000u32;
///敌人出生的位置不能在超人多少范围内
pub const CREATE_ENEMY_DISTANT:f64 = 50f64;
///子弹一运动时间
pub const BULLET_STEP_TIME:u32=0u32;
///子弹一次运动距离
pub const BULLET_STEP_LENGTH:f64=0.5f64;
///敌人一次运动消耗时间
pub const ENEMY_STEP_TIME:u32=5u32;
///敌人一次运动的距离
pub const ENEMY_STEP_LENGTH:f64=0.5f64;
///超人一次运动的时间
pub const SUPER_MAN_STEP_TIME:u32=5u32;
///超人一次运动的距离
pub const SUPER_MAN_LENGTH:f64=0.5f64;

///敌人发送子弹的间隔时间
pub const ENEMY_OPEN_FIRE_SPLIT_TIME:u16 = 2000u16;
///超人发送子弹间隔时间
pub const SUPER_MAN_OPEN_FIRE_SPLIT_TIME:u16 = 10u16;

///爆炸存在时间
pub const EXPLODE_EXIST_TIME:u32 = 100u32;


///计算角度
pub fn calc_angle(start:COORDINATE,target:COORDINATE) -> f64{
    let (x,y) = start;
    let (x1,y1) = target;
    // let y1 = -y1; //游戏坐标系与实际坐标系上下相反
    let a = x1 - x; //a
    let b = y1 - y; //b

    //计算角度
    let mut angle = (b/a).atan();
    if a < 0f64{
        angle = -(consts::PI - angle);
    }
    angle
}

///计算距离
pub fn calc_distance(start:COORDINATE,target:COORDINATE) -> f64{
    let (x,y) = start;
    let (x1,y1) = target;
    let a = x1 - x; //a
    let b = y1 - y; //b
    let c = (a*a + b * b).sqrt();
    return c;
}

///清理失效坐标数据
pub fn clean_coordinate<T:CoordinateTrait>(objs:&mut Vec<T>){
    let mut del_coordinate_list = Vec::new();
    for obj in objs.iter_mut(){
        let (x,y) = obj.coordinate();
        if x > WIN_WIDTH || x <= 0f64 || y > WIN_HEIGHT || y <= 0f64{
            del_coordinate_list.push(obj.coordinate());
        }
    }
    objs.retain(|x|{
        return !del_coordinate_list.contains(&x.coordinate())
    });
}