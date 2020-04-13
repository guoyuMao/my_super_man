//!Draw trait
use piston_window::*;

///Draw
pub trait Draw{
    ///图形画在窗口上
    fn draw (& self,glyphs:&mut Glyphs,c:Context, g:&mut G2d, device:&mut gfx_device_gl::Device);
}

