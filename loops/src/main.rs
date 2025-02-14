//Loop  - it will run until you tell it to stop it
//while
//foor

fn main() {
example4();
    
 
}

fn _example1(){
    //infinite loop
    loop{
        println!("Hello, world!");
     }
}

fn _example2(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {result}" );
}

fn _example3(){
    let count =0;
    
    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining = 10;
        loop{
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }
            remaining -= 1;
        }
    }

}

fn example4(){
    //Looping through a collection
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("the value is: {}", element);
    }


    let letters = ['a', 'b', 'c', 'd', 'e'];
    for letter in letters{
        println!("{letter}");
    }
}