//constants are values that are bound to a name and do not change, they are always immutable
//you can not use mut with constants

fn main() {
    // Constants are declared using the const keyword and must be annotated with a type
    //Constants names are in uppercase with underscores between words
    const _MAX_POINTS: u32 = 100_000;
    println!("Hello, world! {}", _NAME);
}

//is it posible declare a constant in any scope, even outside main function

const _NAME: &str = "Melanie";

