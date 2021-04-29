pub fn raindrops(n: i32) -> String {
    let mut rainresult = String::new();
    let is_factor = |factor: i32| n % factor == 0;

    if is_factor(3) {
        rainresult += "Pling";
    }
    if is_factor(5) {
        rainresult += "Plang";
    }
    if is_factor(7) {
        rainresult += "Plong";
    }

    if rainresult.is_empty() {
        n.to_string()
    } else {
        rainresult
    }
}
