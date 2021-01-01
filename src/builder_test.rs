mod point;
mod bounds;

pub fn get_point_inbound() {

    let mut b_x: bounds::Bounds = bounds::BoundsBuilder::default()
        .min(-1.0)
        .max(1.0)
        .build()
        .unwrap();
    let mut b_y: bounds::Bounds = bounds::BoundsBuilder::default()
        .min(-1.0)
        .max(1.0)
        .build()
        .unwrap();
    let mut b_z: bounds::Bounds = bounds::BoundsBuilder::default()
        .min(-1.0)
        .max(1.0)
        .build()
        .unwrap();
    b_x.rnd();
    b_y.rnd();
    b_z.rnd();

    println!("Bounds x[{},{}] y[{},{}] z[{},{}]",
             b_x.min, b_x.max, b_y.min, b_y.max, b_z.min, b_z.max);

    let mut p: point::Point = point::PointBuilder::default()
        .x(0.0)
        .y(0.0)
        .z(0.0)
        .build()
        .unwrap();

    let point_in_bound: point::Point = loop {
        p.rnd();
        println!("{} {} {}", p.x, p.y, p.z);
        if p.x > b_x.min && p.x < b_x.max &&
           p.y > b_y.min && p.y < b_y.max &&
           p.z > b_z.min && p.z < b_z.max {
            break p;
        }
    }; // again: semicolon needed
    println!("{} {} {}", point_in_bound.x, point_in_bound.y, point_in_bound.z)
}
