
//if else
//![allow(warnings)] use that to ignore warnings
fn main() {
example1(true);
example_2("B");
example_3("yes"); //if and a let statement

}



fn example1( is_car_on:bool ){
    if is_car_on{
        println!("Car is on");
    } else {
        println!("Car is off");
    }

}

fn example_2(letter:&str){
    if letter == "A"{
        println!("A is for Apple");
    } else if letter == "B"{
        println!("B is for Ball");
    } else {
        println!("I don't know that letter");
    }
}

fn example_3(word:&str){
    let answer = if word == "yes" {true} else {false};
    println!("{}", answer);
}