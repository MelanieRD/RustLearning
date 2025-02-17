fn main() {
vectors();
utf8_strings();
hashmaps();
}

fn vectors(){
//Vector should have the same type
//a vector is like a dynamic array that can grow or shrink in size as needed
let _vector:Vec<i32> = Vec::new();

//macro to create a vector
let mut _vector2:Vec<i32> = vec![1,2,3];

_vector2.push(4);
_vector2.push(5);
_vector2.push(6);

println!("{:?}",_vector2);

// FINALLY WE HAVE INDEXING HERE

let _third: &i32 = &_vector2[2];  //Direct indenxing
println!("The third element is {}", _third);


//another way to access elements in a vector is with the get method

let _thirs_get = _vector2.get(2);

}

fn utf8_strings(){

    let mut s1 = String::from("Hellow, ");
    let s2 = String::from("Uw uw uuh");
    s1.push_str("OH oh OH OH"); 
    s1.push_str("Hellow, "); //push_str() appends a string slice to a String
    s1.push_str(&s2); //push_str() appends a string slice to a String
    s1.push('!'); //push() appends a single character to a String
    println!("{}",s1);
    println!("{}",s2);

    //concatenation with + operator
    let s3 = String::from("tic ");
    let s3 = s3 + &s2;
    println!("{}",s3);
}

fn hashmaps(){
    use std::collections::HashMap;
    //it takes two types, one for the type of the keys and the other for the type of the values
    let mut myfirst_hashmap = HashMap::new();
    myfirst_hashmap.insert(String::from("Blue"), 10);
    myfirst_hashmap.insert(String::from("Red"), 20);

    println!("{:?}",myfirst_hashmap);

    //accessing values
    let value = myfirst_hashmap.get("Blue");
    println!("{:?}",value);

    let score = myfirst_hashmap.get("Blue").copied().unwrap_or(0);
    println!("{:?}",score);

    //iterating over a hashmap

    for (key, value) in &myfirst_hashmap {
        println!("{}: {}", key, value);
    }

}
