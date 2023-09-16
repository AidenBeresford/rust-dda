fn main() {
    // we have to initialize a lot of shit here
   
    // line segment endpoints, i'll add user input later
    let x1 : f32 = 3.42;
    let x2 : f32 = 6.84;
    let y1 : f32 = 2.5;
    let y2 : f32 = 10.0;

    // delta x and y, these are assigned with math later
    let dx : f32;
    let dy : f32;

    dx = x2 - x1;
    dy = y2 - y1;

    x_ints(&x1, &y1, &dx, &dy);
    println!("");
    y_ints(&x1, &y1, &dx, &dy);
}

fn x_ints(
    x1: &f32, y1: &f32,
    dx: &f32, dy: &f32
    ) {
    // basically finds every y-value for when x is an integer

    let step_x : f32 = dx / dx.abs();
    let step_y : f32 = dy / dx.abs();

    let y_intercept : f32 = (step_y * -x1) + y1;

    // the range it checks in, will make variable later
    for n in -10..11 {
        print!("X: {}, Y: {}\n", step_x * n as f32, (step_y * n as f32) + y_intercept);
    }

}

fn y_ints(
    x1: &f32, y1: &f32,
    dx: &f32, dy: &f32
    ) {
    // same as x_ints but inverse

    let step_x : f32 = dx / dy.abs();
    let step_y : f32 = dy / dy.abs();

    let x_intercept : f32 = (step_x * -y1) + x1;

    for n in -10..11 {
        print!("Y: {}, X: {}\n", step_y * n as f32, (step_x * n as f32) + x_intercept);
    }
}
