#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}
impl<T,U> Point<T,U> {
    fn x(&self)-> &T{
        &self.x
    }
    fn y(&self)-> &U{
        &self.y
    }
    
}
impl Point<f32,f32> {
    fn distance(&self)->f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let point0 = Point { x: 5, y: 10 };
    let point1 = Point { x: 5, y: 10.8 };
    let point2 = Point { x: 5.2, y: 10.8 };
    let point3 = Point { x: 5.2, y:"OK" };
    
    
    println!("{:#?}{:#?}{:#?}{:#?}",point0,point1,point2,point3);
 
    println!("x1:{},y2:{},y3:{}",point1.x(),point2.y(),point3.y());

    println!("distance: {}", point2.distance());
}