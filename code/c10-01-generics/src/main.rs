struct Point<T> {
    x: T,
    y: T,
}

struct CrossPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> CrossPoint<T, U> {
    fn mixup<V, W>(self, other: CrossPoint<V, W>) -> CrossPoint<T, W> {
        CrossPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 4, y: 3 };
    println!("{}", p.x());
    let p2 = Point { x: 3.2, y: 4.5 };
    println!("{}", p2.distance_from_origin());
    let p3 = CrossPoint { x: 6.4, y: 7.2 };
    let p4 = CrossPoint { x: 2.5, y: 9.4 };
    let p5 = p3.mixup(p4);
    println!("({}, {})", p5.x, p5.y);
}
