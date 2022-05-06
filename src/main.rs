fn main() {
    looper();
}

unsafe fn looper () {
    println!("Count: {}", counter());
    println!("Count: {}", counter());

    let mut num = counter();
    *num = 3;

    println!("Count: {}", counter());
    println!("Count: {}", counter());
    println!("Count: {}", counter());
}

// fn counter() -> u32 {
unsafe fn counter() -> &'static mut u32 {
    static mut X: u32 = 0;
    return &mut X;
}