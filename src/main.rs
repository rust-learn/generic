fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

// implement method named x for Point<T, U>
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implement method distance_from_origin for Point<f32, f32> only
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic type params in struct def DIFFERENT from generic type params in method signatures
impl<T, U> Point<T, U> {
    // declare Generic params V and W in method
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("integer.x() = {}", integer.x());
    println!("float.distance_from_origin() = {}", float.distance_from_origin());

    let mixed = integer_and_float.mix_up(integer);
    println!("mixed.x = {}, mixed.y = {}", mixed.x, mixed.y);
}
