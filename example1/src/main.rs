use crate::Primitive::Triangle;

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1{
        for j in 0..arr.len() - i - 1{
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
struct BigDragon {

    iq: u32,
    name: String
}
impl BigDragon {
    fn new(iq: u32, name: String) -> BigDragon {
        BigDragon { iq, name }
    }
    fn print(&self) {
        println!("IQ {} name {}", self.iq, self.name);
    }
}

fn get_iq(dragon: &BigDragon) -> u32 {
    dragon.iq
}

fn get_name(dragon: &BigDragon) -> String {
    dragon.name.clone()
}


fn playground() {
    println!("Playground, lets make some dragons");
    let dragon1 = BigDragon::new(1, String::from("Morgan"));
    let dragon2 = BigDragon::new(2, String::from("Joe"));
    let dragon3 = BigDragon::new(3, String::from("Bob"));
    println!("dragon1: {} IQ {}", dragon1.name, dragon1.iq);
    println!("dragon2: {} IQ {}", dragon2.name, dragon2.iq);
    println!("dragon3: {} IQ {}", dragon3.name, dragon3.iq);

}

enum Primitive {
    Point {x: f64, y: f64},
    Line { x: f64, y: f64, length: f64 },
    Triangle { a_x: f64, a_y: f64, b_x: f64, b_y: f64, c_x: f64, c_y: f64 },
    Invalid,
}


fn describe(primitive: &Primitive) {
    match primitive {
        Primitive::Point { x, y } => {
            if *x == 0.0 && *y == 0.0 {
                println!("This point is at the origin");
            }
            else{
                println!("This point is at ({}, {})", x, y);
            }
        }
        Primitive::Line { x, y, length } => {
            if *x == 0.0 && *y == 0.0 {
                println!("This primitive is a line of length {} starting at the origin", length);
            }else{
                println!("This primitive is a line of length {} does not start at the origin", length)
            }
        }
        Primitive::Triangle { a_x, a_y, b_x, b_y, c_x, c_y } => {
            let area = calculate_triangle_area(
                (a_x, a_y),
                (b_x, b_y),
                (c_x, c_y),
            );
            println!("This primitive is a triangle that has area {} ", area)
        }
        Primitive::Invalid => {
            println!("This primitive is an invalid type");
        }
    }
}
fn calculate_triangle_area(
    (a_x, a_y): (&f64, &f64),
    (b_x, b_y): (&f64, &f64),
    (c_x, c_y): (&f64, &f64),
) -> f64 {
    let u1 = c_x - a_x;
    let u2 = c_y - a_y;
    let v1 = c_x - b_x;
    let v2 = c_y - b_y;
    0.5 * (u1 * v2 - u2 * v1).abs()
}

fn join(primitive1: &Primitive, primitive2: &Primitive) -> Primitive {
    match (primitive1, primitive2) {
        (
            Primitive::Point { x: x1, y: y1 },
            Primitive::Point { x: x2, y: y2 },
        ) => {
            let length = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
            Primitive::Line {
                x: *x1,
                y: *y1,
                length,
            }
        }
        (
            Primitive::Point { x: px, y: py },
            Primitive::Line {
                x: lx,
                y: ly,
                length: line_length,
            },
        ) => {
            // Calculate the other end of the line
            let end_x = lx + line_length;
            let end_y = ly + line_length ;

            Triangle {
                a_x: *px,
                a_y: *py,
                b_x: *lx,
                b_y: *ly,
                c_x: end_x,
                c_y: end_y,
            }
        }
        (
            Primitive::Line {
                x: lx,
                y: ly,
                length: line_length,
            },
            Primitive::Point { x: px, y: py },
        ) => {
            let end_x = lx + line_length;
            let end_y = ly + line_length ;

            Triangle {
                a_x: *lx,
                a_y: *ly,
                b_x: end_x,
                b_y: end_y,
                c_x: *px,
                c_y: *py,
            }
        }
        _ => Primitive::Invalid,
    }
}





fn main() {
    let mut arr = [5, 3, 2, 6, 9];
    bubble_sort(&mut arr);
    println!("arr: {:?}", arr);
    playground();

    let point1 = Primitive::Point { x: 1.0, y: 2.0 };
    let point2 = Primitive::Point { x: 2.0, y: 3.0 };
    let line1 = Primitive::Line { x: 1.0, y: 2.0, length: 3.0 };
    let triangle = Triangle { a_x: 1.0, a_y: 1.0, b_x: 1.0, b_y: 2.0, c_x: 3.0, c_y: 1.0 };

    describe(&point1);
    describe(&line1);
    describe(&triangle);

    let line2 = join(&point1, &point2);
    describe(&line2);

    let triangle2 = join(&point1, &line1);
    describe(&triangle2);

    let triangle3 = join(&line1, &point1);
    describe(&triangle3);

    let invalid = join(&line1, &triangle);
    describe(&invalid);


}


