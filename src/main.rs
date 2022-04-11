enum Color {
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            _red => println!("Color : {:?}", "Red"),
        }
    }
}

struct Dimension {
    l: u64,
    w: u64,
    h: u64,
}

impl Dimension {
    fn new(l: u64, w: u64, h: u64) -> Self {
        Self { l, w, h }
    }
    fn print(&self) {
        println!("length:{:?} width:{:?} height:{:?}", self.l, self.w, self.h)
    }
}

struct ShippingBox {
    color: Color,
    weight: u64,
    dimension: Dimension,
}

impl ShippingBox {
    fn new(color: Color, dimension: Dimension, weight: u64) -> Self {
        Self {
            color,
            dimension,
            weight,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimension.print();
        println!("Weight : {}kg", self.weight)
    }
}

fn main() {
    let shipping_box = ShippingBox::new(Color::Red, Dimension::new(3, 5, 6), 2);
    shipping_box.print();
}
