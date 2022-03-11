fn five() -> i32 {
    5
}

fn main() {
    another_function(4, 'n');
    let x = five();

    println!("The value of x is: {}", x);
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
