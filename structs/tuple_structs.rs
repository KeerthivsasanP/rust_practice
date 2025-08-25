struct Color(usize, usize, usize);
struct Point(usize, usize, usize);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //Destructing tuple structs
    let Point(x, y, z) = origin;
    let Color(r, g, b) = black;

    println!("Point x:{x} y:{y} z:{z} ");
    println!("Color r:{r} g{g} b:{b} ");
}

