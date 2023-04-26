fn main() {
    let k = 25;
    let m = 27;
    let n = 15;
    let total = k + m + n;
    let mut prob = 0.0;
    prob += k as f64 / total as f64;
    prob += m as f64 / total as f64 * (k as f64 / (total - 1) as f64);
    prob += m as f64 / total as f64 * ((m - 1) as f64 / (total - 1) as f64) * 0.75;
    prob += 2.0 * (m as f64 / total as f64 * (n as f64 / (total - 1) as f64) * 0.5);
    prob += n as f64 / total as f64 * (k as f64 / (total - 1) as f64);
    println!("{}", prob);
}
