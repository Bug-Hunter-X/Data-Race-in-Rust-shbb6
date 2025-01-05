fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &mut x; 
    *z = 15; 
    println!("x = {}", x);
}