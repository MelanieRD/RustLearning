fn main() {
    println!("Hello, world!");
    let _S:i32 = 5;
    if _S < 10{
        println!("_S is less than 10");}

    //Primitive data types
    //int, float. bool, char

    // Integer types: posiives and negatives: (i8, i16, i32, i64, i128, isize) and unsigned, just positives: (u8, u16, u32, u64, u128, usize)
    //range: i32: -2^31 to 2^31 - 1, u32: 0 to 2^32 - 1
    //range: i64: -2^63 to 2^63 - 1, u64: 0 to 2^64 - 1
    //range: i128: -2^127 to 2^127 - 1, u128: 0 to 2^128 - 1
    //isize and usize: depends on the architecture of the computer, 32 bits or 64 bits

    let x: i32 = 10;
    let y: u64 = 100;

    println!("Singned Integer: {}", x);
    println!("Unsingned Integer: {}", y);

    //Floating point types: f32, f64
    let pi: f32 = 3.14159265359;
    println!("Value of pi: {}", pi);

    //Boolean type: bool
    let is_happy: bool = true;
    println!("Is happy: {}", is_happy);

    //char type: char
    let letter: char = 'a';
    println!("Letter: {}", letter);
    //--------------------------------------------------------- Compound Data Types ------------------------------------------ 
    //arrays, tuples, slices, strings

    //----------------arrays: fixed size, same data type
    let numbers: [i32;8] = [1, 2, 3, 4, 5,7,10,49]; // you have to espexify the size of the array and the size of the elements [size; elements]
    println!("Array: {:?}", numbers);

    let names:[&str; 3] = ["John", "Jane", "Doe"]; //$str is a string reference, not teh string itself
    println!("Array: {:?}", names); //{:?} Debugable format
    println!("Array: {}", names[2]);

    //-----------------tuples: fixed size, different data types

    // Example let human:(String, i32, bool) = ("Melanie", 24, true); 
    // println!("Human Tuple: {:?}", human); //this will give an error because the tuple is not a string, it is a tuple, a string slice
    //we have to convert "Melanie" to a string, we can do this by using the to_string() method
    let human:(String, i32, bool) = ("Melanie".to_string(), 24, true);
    println!("Human Tuple: {:?}", human);

    // we can also add compound datatypes to a tuple
    let human2:(String, i32, bool, [i32; 3]) = ("Melanie".to_string(), 24, true, [1, 2, 3]);
    println!("Human Tuple: {:?}", human2);


    //------------------Slices: reference to a sequence of elements in a collection

    let numbers_slices: &[i32] = &[1,4,6,8,4];
    println!("Numbers Slices: {:?}", numbers_slices);


    let names_slices :&[&String] = &[&"John".to_string(), &"Jane".to_string(), &"Doe".to_string()];
    println!("Names Slices: {:?}", names_slices);

    let fruits_slices: &[&str] = &["apple", "banana", "orange"];
    println!("Fruits Slices {:?}",fruits_slices);

    /* -------------------------  String vs String Slices (&str) -------------------------------------
        String: a growable, heap-allocated data structure
    */

    let mut words:String = String::from("Hello"); //let mut words:String = "Hello".to_string();
    words.push_str(", World!");
    words.push_str(", I am Melanie");
    println!("Words: {}", words);	


    //String Slices: a reference to a sequence of bytes in a string, is not mutable  $str

    let string:String  = String::from("Hello, World!");
    let string_slice: &str = &string[0..5]; //this will take the first 5 characters of the string
    println!("String Slice: {}", string_slice);

    // Heap vs Stack

    //Heap is where the data is stored, it is slower than the stack
    //Stack is where the pointers to the data are stored, it is faster than the heap (it can't have any mutable data)


    //------------------------------------------------------------ Functions --------------------------------------------------------
    string_upper_carser("mi nombre es");
    number_multiplier(10, 20);
    number_multiplier(-5,5);

    let x = {
        let price =  5;
        let quantity = 10;
        price * quantity
    };

    println!("The value of x is: {}", x); 

    /*
    how you can see, the value of x is the result of the last line of the block, 
    is not need it to put a semicolon at the end of the block and the return statement is not needed
    */

    let result = function_with_return_value(10, 20); // 
    println!("Result: {}", result);

    let string_idk = "Hello, Rust!";
    let string_idk2 = String::from("Hello, Rust!"); //para que funcione cel metodo debe ser una referencia a un string, no un string y por esos e utiliza &String
    two_sum(&string_idk2);

    } //end of the main function-----------------------------------


    fn string_upper_carser(string_to_upper_case: &str){
        println!("Hello, Rust! This is my new function, this is your string: {:?}", string_to_upper_case.to_uppercase());
    // println!("Hello, Rust! This is my new function, this is your string: {:?}", stringToUpperCase.to_uppercase().to_string()); // porque no se pude?

    }

    fn number_multiplier(first_number:i32, second_number:i32){
        let result = first_number * second_number;
        println!("Result: {}", result);

    }

    fn function_with_return_value(number1:i32, number2:i32) -> i32{
        //I need to specify the return type of the function using the -> symbol
        number1 + number2
    }
    
    fn two_sum(s:&String){
        let string_length = s.len();
        println!("The length of the string is: {}", string_length);
    
    }
