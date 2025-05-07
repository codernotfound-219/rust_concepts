mod advanced;

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let point = Point { x:10, y:40 };
    println!("p.x = {}, p.y = {}", point.get_x(), point.get_y());

    let adv_point_1 = advanced::Point { x:20, y:"Hello" };
    let adv_point_2 = advanced::Point { x:" world", y:"iplt" };

    let adv_point_3 = adv_point_1.mixup(adv_point_2);
    println!("p.x = {}, p.y = {}", adv_point_3.x, adv_point_3.y);
}
