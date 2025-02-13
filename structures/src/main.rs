  

 // we can have only one mutable reference to a value or any number of immutable references, but not both at the same time
//Aqui estamos nuevamente utilizando el ejemplo de las referencias, pero ahora con una estructura

fn main() {
    explanation_with_reference();
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