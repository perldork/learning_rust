#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn has_area(&self) -> bool {
        self.area() > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    calculate_area_no_structs();
    calculate_area_with_tuple();
    calculate_area_with_structs();
    calculate_area_with_methods();
    can_one_rect_hold_another();
}

fn can_one_rect_hold_another() {

    println!("Can one Rectangle contain another Rectangle inside of itself?");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("\tCan rect1 {:?} hold rect2 {:?}? {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("\tCan rect1 {:?} hold rect3 {:?}? {}", rect1, rect3, rect1.can_hold(&rect3));

}


fn calculate_area_with_methods() {
    println!("Calculate area 4: with methods");

    let scale = 2;
    let width: u32 = 30;

    let rect = Rectangle {
        width: dbg!(width.pow(scale)),
        height: 50,
    };

    println!(
        "\tThe area of the rectangle ({}, {}) is {} square pixels.",
        rect.width(),
        rect.height(),
        rect.area()
    );

    dbg!(&rect);

    if rect.has_area() {
        println!("\tThe rectange has a non-zero area of {}", rect.area());
    } else {
        println!("\tThe rectange has a zero area");
    }

}

fn calculate_area_with_structs() {
    println!("Calculate area 3: with structs");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "\tThe area of the rectange {:?} is {} square pixels.",
        rect,
        area_structs(&rect)
    );

    println!("\tUsing dbg! macro to print the struct and code location\n\t");
    dbg!(&rect);
}

fn calculate_area_no_structs() {

    println!("Calculate area 1: no structs");

    let width1 = 30;
    let height1 = 50;

    println!(
        "\tThe area of the rectangle (width: {}, height: {}) is {} square pixels.",
        width1,
        height1,
        area(width1, height1)
    );
}

fn calculate_area_with_tuple() {
    println!("Calculate area 2: with tuple");

    let rect = (30, 50);

    println!(
        "\tThe area of the rectangle (width, height) {:?} is {} square pixels.",
        rect,
        area_tuple(rect)
    );
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
