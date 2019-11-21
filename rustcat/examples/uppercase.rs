// cargo run -q --example uppercase

fn uppercase() {
    let s = "apple";
    let apple = s.to_uppercase();
    println!("{}", apple);
}

fn main() {
    uppercase();
}
