fn main() {
    let mut s = String::from("hello");
    //mutate string
    s.push_str(", world!");
    println!("{}", s)
}
