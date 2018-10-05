fn match01(a: i32, b: bool) {
    let x = (a, b);
    match x {
        (i, true) if 20 <= i && i <= 26 => {println!("Case a");},
        (_, true) => {println!("Case b");},
        (i, _) if 40 <= i && i < 50 => {println!("Case c");},
        _ => {println!("Case d")}
    }
}

fn main() {
    match01(51, true);
    match01(26, true);
    match01(51, false);
    match01(45, false);
    match01(45, true);
}