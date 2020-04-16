//! config-comm
use std::option::Option;
use piston_window::G2dTexture;

///任务所占屏幕百分比
pub const PERSON_SIZE:f64 = 0.05f64;

///字体大小
pub const FONT_SIZE:u32 = 32u32;

///坐标
pub type COORDINATE = (f64,f64);

///窗口宽
pub const WIN_WIDTH:f64 = 1000f64;
///窗口高
pub const WIN_HEIGHT:f64 = 500f64;

///窗口大小
pub type WIN_SIZE = [f64;2];

///子弹图片
pub static mut BULLET_TEXTURE:Option<G2dTexture> = None;


///基础速度
pub const BASE_SPEED:f64  = 0.3f64;
///加速度
pub const ACCELERATION:f64 = 0.5f64;

///子弹速度
pub const BULLET_SPEED:f64 = 0.5f64;