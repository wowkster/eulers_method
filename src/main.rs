fn main() {
    let dy_dx = |x: f32, y: f32| (2f32 * x * x) + y;
    let f_0 = -1f32;

    let n = 4;
    let range = 0..2;

    let delta_x: f32 = (range.end - range.start) as f32 / n as f32;

    let mut y = f_0;
    let mut x = range.start as f32;

    println!("== EULER'S METHOD CALCULATOR ==");

    println!();

    println!("|-----------|-----------|---------|--------|----------------------------|");
    println!("|     x     |     y     |  dy/dx  |   Δx   | y_new = y_old + dy/dx * Δx |");
    println!("|-----------|-----------|---------|--------|----------------------------|");

    while x < range.end as f32 {
        let y_old = y;

        let dy_dx = dy_dx(x, y);

        let y_new = y_old + dy_dx * delta_x;

        println!("|{x:>11}|{y_old:>11}|{dy_dx:>9}|{delta_x:>8}|{y_new:>28}|");
        println!("|-----------|-----------|---------|--------|----------------------------|");

        y = y_new;
        x += delta_x;
    }

    println!("|{x:>11}|{y:>11}|       - |      - |                          - |");
    println!("|-----------|-----------|---------|--------|----------------------------|");

    println!();

    println!("The value of f(x) evaluated at f({}) = {}", range.end, y);
}
