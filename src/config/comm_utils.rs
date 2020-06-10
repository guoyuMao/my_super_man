//! comm_utils 通用公共类

use super::config::{COORDINATE,CoordinateTrait,WIN_WIDTH,WIN_HEIGHT,WinSize};
use std::f64::consts;
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

///计算两点中心
pub fn calc_core(c1:COORDINATE,c2:COORDINATE) ->(f64,f64){
    let (ax,ay) = c1;
    let (bx,by) = c2;
    ((ax+bx)/2f64,(ay+by)/2f64)
}

///是否碰撞
pub fn is_collide(c1:COORDINATE,s1:WinSize,c2:COORDINATE,s2:WinSize) -> bool{
    let (ax, ay, ar) = calc_xyr(c1, s1);
    let (bx, by, br) = calc_xyr(c2, s2);
    let center_distance = calc_center_distance(ax, ay, bx, by); //圆心距
    return if center_distance < (ar+br) {
        true
    }else{
        false
    }
}

///计算图形原点及内切圆半径
/// c : 原始坐标
/// s : 尺寸
fn calc_xyr(c:COORDINATE,s:WinSize) -> (f64,f64,f64){
    let (x,y) = c;
    // let (x,y) = (x+s[0]/2f64,y+s[1]/2f64); //坐标中心
    return if s[0] < s[1] {
        (x, y, s[1] / 2f64)
    } else {
        (x, y, s[0] / 2f64)
    }
}


///计算圆心距离
fn calc_center_distance(ax:f64,ay:f64,bx:f64,by:f64) -> f64{
    let pow_xy = (ax-bx).abs().powi(2) + (ay-by).abs().powi(2);
    pow_xy.sqrt()
}

#[test]
fn test(){
    println!("{}",calc_center_distance(2f64, 2f64, 4f64, 4f64));
}