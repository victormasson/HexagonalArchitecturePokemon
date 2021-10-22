mod domain;
mod repositories;

fn main() {
    let v1 = "hello".to_string();
    let v2 = format!("v2 {}", v1);

    println!("{}", v1);
    println!("{}", v2);
}
