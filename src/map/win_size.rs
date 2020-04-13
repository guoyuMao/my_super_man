//! 窗口mod

use super::COORDINATE;

///窗口数据结构
#[derive(Debug)]
pub struct WinSize{
    ///coordinate
    pub coordinate:COORDINATE,
    ///width
    pub width:f64,
    ///height
    pub height:f64,
}

impl WinSize{
    ///默认初始化
    pub fn init() -> WinSize{
        WinSize{
            coordinate:[0f64,0f64],
            width:650f64,
            height:350f64,
        }
    }

    ///新建窗口
    pub fn new(coordinate:COORDINATE,
               width:f64,
               height:f64,) -> WinSize{
        WinSize{
            coordinate,
            width,
            height,
        }
    }
}
