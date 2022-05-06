fn main() {
    static age: u32 = 90;

    let var = "Hello world";
    let num = 32;
    let num2 = 32u8;

    // completeley different from JS's const
    const NAME: &str = "CANARIASJS";
    // at compile time usages of NAME get replaced by the value
    // use const for values that can be determined at compile time

    println!("Hello, world!");
}
