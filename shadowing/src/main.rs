//Shadowing is when you declare a variable with the same name as a previous variable. 
//This is useful when you want to change the type of a variable or reassign a variable.
//shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we forget to reassign the variable.

fn main() {
    example1();
    example2();

}

fn example1() {
    let  x = 5;
    let  x = x + 1;
    let  x = x * 2;
    println!("The value of x is: {}", x);

    {
        let x = x* 2;
        println!("The value of x is: {}", x);
    }
}


fn example2() {
    // in this example, the variable spaces is first declared with the value of "   ", and then shadowed with an integer value.
    // you could think that instead of shadowing, we can use mut to change the value of the variable. But it will give yoy an erroy because you
    // can't change the type of a variable with mut.
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}

