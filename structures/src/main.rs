  //Structs are a way to create custom types that can group multiple fields together
    //Structs are similar to tuples, but with a key difference: the fields of a struct have names, where the fields of a tuple have only their position.
 // we can have only one mutable reference to a value or any number of immutable references, but not both at the same time
//Aqui estamos nuevamente utilizando el ejemplo de las referencias, pero ahora con una estructura
#![allow(warnings)]

fn main() {  
   // explanation_with_reference();
    explanation_tuple_struct();
    let animal_kawai = return_struct_from_function();
    println!("The animal is {}", animal_kawai.name);


    //Creating a struct instance by given values and return it in a function

    let animal_kawai2 =  struct_animal_creator("Melanie".to_string(), "Purple".to_string(), 100);
    println!("The animal is {} it color is {}", animal_kawai2.name, animal_kawai2.color);

    // Creating a struct instance by other instance  // <-------------------------- ATENTION TO IT, THE ... is used to copy the values of the other instance

    let animal_kawai3 = AnimalKawai{
        name: String::from("Juliet"),
        ..animal_kawai2 // <--------------- WE COPY EVERYTHING FROM ANIMAL_KAWAI2 EXCEPT THE NAME
    };
    
    println!("The animal is {} it color is {}", animal_kawai3.name, animal_kawai3.color);

}

fn explanation_tuple_struct(){
      //--------------------------------------------- TUPLE  ---------------------------------------------
      //tuple is a collection of values of different types that can be grouped together, but it does not have a name for each value like a struct
      let person_tuple = ("Melanie", 30);
        
        //--------------------------------------------- TUPLE STRUCT  ---------------------------------------------

        //tuple struct is a way to create a custom data type that is a tuple, it doesnt have any names for the fields, but it has a name for the type

        struct Color(i32, i32, i32); // <--- tuple struct with 3 fields of type i32
        let black = Color(0, 0, 0); // <--- instance of Color tuple struct
        let white = Color(255, 255, 255); // <--- instance of Color tuple struct

                //--------------------------------------------- UNIT-LIKE STRUCT --------------------------------------------- TO STUDY LATER
        //unit-like struct is a way to create a custom data type that has no fields, it is similar to a tuple struct, but without any fields
        




      // --------------------------------------------- STRUCTS ---------------------------------------------
      struct PersonStructure { // the name of struct is in upper camel case, wiothout _
            id: u32,
          name: String,
          last_name: String,
          gender: String,
          age: u32,
          role_id: u32,
          
      };

      struct RoleStructure{
        id: u32,
            role: String,
            description: String,
            salary: u64,
            state: bool,

      }
      
      //instance of Role 
        let role1 = RoleStructure {
                id : 1,
            role: String::from("Developer"), // <--- String::from() is a way to create a new String from a string literal
            description: "Develops software".to_string(),
            salary: 10,
            state: true,
            };

      //instance of person
      let mut person1 = PersonStructure { // <--- mutable instance of Person
            id : 1,
          name: "Melanie".to_string(),
          last_name: "HM".to_string(),
          gender: "F".to_string(),
          age: 24, 
          role_id: 1,
        };

        // If i want to update some information of the person
        person1.gender = String::from("TITAN");  
        println!("The person is a {}!", person1.gender); 


}


fn explanation_with_reference(){
    let mut account = BankAccount {
        account_owner: "Melanie".to_string(),
        balance: 100.0,
    };

    //inmutable borrow to check the balance
    account.check_balance();

    //mutable borrow to withdraw money
    account.withdraw(18.0);
    account.check_balance();
}

// Struct a data structure that allows you to group multiple fields together under one name
struct BankAccount {
    account_owner: String,
    balance: f64,
}
//impl es una implementación de métodos para una estructura (struct).
//Permite definir funciones que están asociadas con la estructura.

impl BankAccount {
    // I want to ensure that we cannot simultaneously have mutable access to the account to update the balance and inmutable access to reding the account owners name
    //for that we use &mut self and &self
    
    
    //metodos de bankaccount
    fn withdraw(&mut self, amount: f64) {
        //&mut self significa que mientras la función withdraw se está ejecutando, nadie más puede acceder a account mutablemente o inmutablemente.
        //*SeLf refiere a la propia instancia de la estructura. self pasa la estructura por valor, lo que significa que toma la propiedad de la instancia y la función ya no puede usarse después. */

        println!("Withdrawing ${} from account owned by {}", amount, self.account_owner);
        self.balance -= amount;
    }

    // we want that while we are checking the balance, we do not want another part of our code modifying the balance which has a mutable access
    fn check_balance(&self) {
        println!("The account owned by {} has a balance of ${}", self.account_owner, self.balance);
    }

    /*
    si utilizo self en vez de &self, me da error porque self toma la propiedad de la instancia y la función ya no puede usarse después.
    el valor de self se mueve a la función, por lo que no se puede usar después de llamar a la función.
    
    fn check_balance(self) {
        println!("The account owned by {} has a balance of ${}", self.account_owner, self.balance);
    }
     */

}

struct AnimalKawai{
    name:String,
    color:String,
    kawai_level:u32,
  }

fn return_struct_from_function()->AnimalKawai{
    //returning a struct from a function
    // not need to use return, the last line of the function is the return value
    // not need to save here in a variable, because the function is returning the struct

AnimalKawai {
    name: String::from("Pikachu"),
    color: String::from("Yellow"),
    kawai_level: 100,
}
 

} 


fn struct_animal_creator(_name:String, _color:String, kawai_lvl:u32 )->AnimalKawai{
    // not need to use return, the last line of the function is the return value
    // not need to save here in a variable, because the function is returning the struct

     AnimalKawai{
        name: _name,
        color: _color,
        kawai_level: kawai_lvl,
    }
  
}