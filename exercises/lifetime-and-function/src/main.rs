#![allow(dead_code)]

/*
Goals:
- コンパイルできるように、Point構造体にaddを実装してください
- コメントアウトされている[1], [2]を戻し、コンパイルできないことを確認してください
- 適切なライフタイムパラメータを追加して、コンパイルできるようにしてください
- コメントアウトされている[3], [4]を戻し、コンパイルエラーが発生することを確認してください
*/

fn main() {
    points();
    area1();
    /* [3]
    area2();
    */
}

fn points() {
    let from = Point::new(10.0, 10.0);
    let diff = Point::new(5.0, 10.0);
    let to = from.add(&diff);
    println!("({}, {})を({}, {})動かした場所は({}, {})です",
             from.x,
             from.y,
             diff.x,
             diff.y,
             to.x,
             to.y);
}

fn area1() {
    let top = Point::new(10.0, 10.0);
    let diff = Point::new(5.0, 10.0);
    let bottom = top.add(&diff);
    let rect = Rect::new(&top, &bottom);
    println!("area = {}", rect.area());
}

/* [4]
fn area2() {
    let top = Point::new(10.0, 10.0);
    let rect;
    {
        let diff = Point::new(5.0, 10.0);
        let bottom = top.add(&diff);
        rect = Rect::new(&top, &bottom);
    }
    println!("area = {}", rect.area());
}
*/
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
    fn add(&self, offset: &Point) -> Point {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

/* ライフタイムが1個のときはかかなくていい */
/* ライフタイムパラメタはライフタイムの依存関係を示すらしい？？ */
struct Rect<'a> {
    top_left: &'a Point,
    bottom_right: &'a Point,
}

impl<'a> Rect<'a>{
    fn new(top_left: &'a Point, bottom_right: &'a Point) -> Rect<'a> {
        Rect {
            top_left: top_left,
            bottom_right: bottom_right,
        }
    }
    fn area(&self) -> f32 {
        let dx = (self.top_left.x - self.bottom_right.x).abs();
        let dy = (self.top_left.y - self.bottom_right.y).abs();
        dx * dy
    }
}
