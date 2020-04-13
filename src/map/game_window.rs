//! 游戏窗口
//!
use super::win_size::WinSize;
use crate::config::colour;

pub use piston_window::*;

///游戏窗口
pub struct MainWindow{
    ///win_size
    pub win_size:WinSize,
    ///background
    pub background:colour::Colour,
}

impl MainWindow{
    ///new main window
    pub fn new() -> MainWindow{
        let win_size = WinSize::init();
        MainWindow {
            win_size: win_size,
            background: colour::RED,
        }
    }
}

///画主图
impl super::draw::Draw for MainWindow{

    ///画图
    fn draw (& self, glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device){
        let ref win_size:WinSize = self.win_size;
        let ref coordinate = win_size.coordinate;
        //画title区
        let pos:[f64;4] = [
            coordinate[0],
            coordinate[1],
            win_size.width,
            win_size.height,
        ];
        Rectangle::new(self.background).draw(pos, &c.draw_state, c.transform, g);
            // // Update glyphs before rendering.
            // glyphs.factory.encoder.flush(device);
    }
}