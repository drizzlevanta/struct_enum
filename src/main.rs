fn main() {
    println!("Hello, world!");
    println!("Area is: {}", cal_area(2.0, 4.321));
    println!("Area2 is: {}", cal_area2((2.0, 4.321)));

    let dim = Dimension {
        width: dbg!(1.0 * 2.0),
        height: 4.321,
    };
    println!("Area3 is: {}", cal_area3(&dim));
    println!("dim is: {:#?}", dim);
    dbg!(&dim);
    dbg!(&dim.area());
    println!("area is {}", dim.area());

    if dim.width() {
        println!("width is {}", dim.width);
    }

    //can hold
    let dim1 = Dimension {
        width: 30.0,
        height: 50.0,
    };

    let dim2 = Dimension {
        width: 10.0,
        height: 40.0,
    };

    let dim3 = Dimension {
        width: 60.0,
        height: 45.0,
    };

    println!("Can dim1 hold dim2?{}", dim1.can_hold(&dim2));
    println!("Can dim1 hold dim3?{}", Dimension::can_hold(&dim1, &dim3));

    //call associated function with struct name
    let sq = Dimension::square(3.0);
    println!("sq is: {:?}", sq);
}

fn cal_area(w: f64, h: f64) -> f64 {
    w * h
}

fn cal_area2(dimension: (f64, f64)) -> f64 {
    dimension.0 * dimension.1
}

fn cal_area3(dimension: &Dimension) -> f64 {
    let height = dimension.width;
    println!("{height}");
    dimension.width * dimension.height
}

#[derive(Debug)]
struct Dimension {
    width: f64,
    height: f64,
}

//methods
impl Dimension {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0.0
    }

    fn can_hold(&self, other: &Dimension) -> bool {
        self.width > other.width && self.height > other.height
    }

    //constructor
    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
