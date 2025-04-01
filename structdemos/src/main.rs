#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(self) -> u32 {
        self.width * self.height
    }
    fn width(&self) ->bool {
        self.width>0
    }
}
impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rect {
    fn square(size:u32)->Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width:20,
        height:40
    };
    let rect3 = Rect {
        width:40,
        height:40
    };
    let rect4 = Rect::square(5);
    println!("{:#?}", rect4);
    println!("该矩形的宽为非零值: {},具体为：{}", rect1.width(),rect1.width);
    println!("Rect1 能否放下Rect2？{}", rect1.can_hold(&rect2));
    println!("Rect1 能否放下Rect3？{}", rect1.can_hold(&rect3));
    println!(
        "面积为:{}",
        rect1.area()
    );
}