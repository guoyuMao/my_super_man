//! 人员
use crate::config::{config, colour};
use crate::weapon;
use piston_window::*;
use crate::weapon::bullet::Bullet;

///爆炸类型
pub enum ExplodeType{
    ///类型1
    type1,
    ///类型2
    type2
}

///敌人
pub struct Explode{
    ///坐标点
    pub coordinate: config::COORDINATE,
    ///尺寸
    pub win_size: config::WinSize,
    explode_type:ExplodeType,
    ///停留时间
    pub exist_time:u32, //存在时间
}

///爆炸
impl Explode{
    ///新爆炸
    pub fn new(coordinate: config::COORDINATE, explode_type:ExplodeType) -> Explode{
        let width:f64 = config::WIN_WIDTH * config::PERSON_SIZE;
        let height:f64 = config::WIN_HEIGHT * config::PERSON_SIZE;
        Explode{
            coordinate:coordinate,
            win_size:[width,height],
            explode_type,
            exist_time: config::EXPLODE_EXIST_TIME,
        }
    }

    ///执行
    pub fn exec(&mut self){
        self.exist_time = self.exist_time - 1;
    }
}

///画敌人
impl  crate::map::draw::Draw for Explode{

    ///画图
    fn draw (& self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        let ref win_size = self.win_size;
        let ref coordinate = self.coordinate;

        unsafe {
            let mut texture: &Option<G2dTexture> = &None;
            match self.explode_type{
                ExplodeType::type1 => texture = &config::EXPLODE_TEXTURE1,
                ExplodeType::type2 => texture = &config::EXPLODE_TEXTURE2,
                _ => {}
            }
            if let Some(image) = texture{
                // 获取图片尺寸
                let (width, height) = image.get_size();
                let (x,y) = self.coordinate;
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