#![allow(warnings)]
fn main() {
    //aproach 1 option
    option_example();

    //aproach 2 Result
    result_example();



    //Example Option
   let result_operation1 = divider_example(100.0,0.0);

     match result_operation1{
         Some(value) => println!("The result of the division is: {}", value), // that mean that if some value is present, it will print the value
         None => println!("The division could not be done, Cannot divide by zero"), //if there is no value, it will print this message
    }

   let result_operation2 = divider_example(100.0,2.0);

    match result_operation2{
        Some(value) => println!("The result of the division is: {}", value),
        None => println!("The division could not be done, Cannot divide by zero"),
   }

   let result_operation3 = divider_example(100.0,3.0);

   match result_operation2{
    Some(value) => println!("The result of the division is: {}", value),
    None => println!("The division could not be done, Cannot divide by zero"),
    }

    //Example Result


    let result_operation4 = multiply(0, 2);
    match result_operation4{
        Ok(value) => println!("The result of the multiplication is: {}", value),
        Err(error) => println!("The multiplication could not be done, {}", error),
    }


    // Finally I can do this as well

    match multiply(2, 2){
        Ok(value) => println!("The result of the multiplication is: {}", value),
        Err(error) => println!("The multiplication could not be done, {}", error),
    }
}



fn option_example() {

    //it is a generic enum, that is used to represent the absence of a value, its going to avoid the null pointer exception
    //The option is going to check something and return a value with that type if it is present, otherwise it will return None
    //thats how you define a generic Option type
    enum Option<T>{
        //we have 2 variants, Some and None
        //Some is used to store the value of type T, represent a value
        Some(T), 
        //None is used to represent the absence of a value
        None, 
    }
}

fn result_example(){
    enum Result<T, E>{
        //we have 2 variants, Ok and Err
        //Ok is used to store the value of type T, represent a value, if it succeded its going to return Ok with the type value
        Ok(T),
        // in case of an error its going to return Err 
        Err(E),
    }
}


fn divider_example(numerator:f64, denomenator:f64)->Option<f64>{
    if denomenator == 0.0{
        None
    }else{
        Some(numerator/denomenator)
    }
}

fn multiply(first_number:i32, second_number:i32)-> Result<i32, String>{
    if first_number == 0 || second_number == 0{
        Err(String::from("The programmer doesn't want you to multiply by zero, Srry, I need to practice and that was the first thing that came to my mind"))
        // Err("Cannot multiply by zero".to_string()) <-- this is the same as the line above
    }else{
        Ok(first_number*second_number)
    }
}