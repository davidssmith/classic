/// Calculate pi from series approximation



fn calc_pi(n: usize) {
    //num: f64 = 4.0;
    let mut denom: f64 = 1.0;
    let mut pi1: f64 = 0.0;
    let mut pi2: f64 = 0.0;
    let mut pi3: f64 = 0.0;
    let mut pi4: f64 = 0.0;
    for _ in 0..(n/2) {
        pi1 += 4.0 / denom;
        pi2 += 4.0 / (denom + 2.0);
        pi3 += 4.0 / (denom + 4.0);
        pi4 += 4.0 / (denom + 6.0);
        denom += 8.0;
    }
    let pi = pi1 - pi2 + pi3 - pi4;
    println!("est pi: {} (error: {:.3e})",pi, pi - std::f64::consts::PI);
}



fn main() {
    calc_pi(100_000_000);
}
