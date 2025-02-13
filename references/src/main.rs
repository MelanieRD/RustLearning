//references == borrowing
//safety refers to the fact that the compiler can guarantee that references will never be dangling references


//to ceate a reference, we use the & symbol
/*
    let x = 5;
    let r = x; you are transfering de ownership of x to r x, is not a reference to x, not good for memory. USER REFERENCES
*/
fn main() {
explanation();



   
}



fn explanation(){
        //MODFYING BY REFERENCE

    //not mutable reference
    let _x:i32 = 5;
    let _r= &_x; //this is a reference to x, is not mutable. ok
    println!("_r value: {}", _r);

    //mutable reference
    //if i want to create a mutable reference the owner have to be mutable too!!
    let mut _y = 5;
    let _p = &mut _y; //this is a mutable reference to y, is mutable. _p its a way to access the value of _y and modify it
    *_p += 10; //this is how you can change the value of the reference _y
    *_p -= 4;

    // _y += 10; //this is the same of *_p += 10
    
    // the * symbol is called the dereference operator, it allows us to dereference a reference so we can modify the value it points to
    // if we try to modify the value of a reference without using the dereference operator, we will get a compilation error, 
    // because we are not allowed to modify the value of a reference directly.
    
    // si pongo esto primero println!("_y value: {}", _y); me da error porque no puedo tener una referencia mutable y una inmutable al mismo tiempo, es decir, no puedo tener _p y _y al mismo tiempo
    //pero si lo pongo despues de imprimir _p, no hay problema
    println!("_p value: {}", _p);
    println!("_y value: {}", _y);

    /*
    si println!("_y value: {}", _y); estuviese de primero...
    _p es una referencia mutable a _y, lo que significa que _y está prestado de forma mutable y no puede ser accedido directamente hasta que _p deje de usarse.
    println!("_y value: {}", _y); intenta acceder a _y de forma inmutable mientras todavía hay una referencia mutable activa (_p).
    Rust no lo permite, ya que podría generar condiciones de carrera y datos inconsistentes.

    - Esto significa que puedes cambiar su valor directamente, pero solo cuando no hay referencias activas. Ejemplo:
   
    fn main() {
    let mut x = 5;
    x += 1;  // ✅ Modificamos directamente, sin referencias
    println!("{}", x);
    } 

    - Cuando tomas una referencia mutable (&mut), Rust te impide usar la variable original hasta que la referencia termine. Ejemplo:
    
    fn main() {
    let mut x = 5;
    let r = &mut x;  // 🔴 Ahora `x` está "prestado" y NO se puede usar directamente

    println!("{}", x); // ❌ Error: `x` no se puede acceder mientras `r` exista
    }

    SOLUCION:

    fn main() {
    let mut x = 5;

    {  // Creamos un bloque para limitar el tiempo de vida de `r`
        let r = &mut x; 
        *r += 1;  // ✅ Modificamos `x` a través de `r`
    } // 🔴 Aquí `r` deja de existir, y `x` vuelve a estar disponible

    println!("{}", x); // ✅ Ahora `x` se puede usar de nuevo
    }



     */


   // Cuando usas &mut, estás modificando directamente el valor original. Esto es parte del sistema de propiedad y referencias de Rust, 
   //que garantiza que no haya referencias mutables conflictivas al mismo tiempo, evitando condiciones de carrera y errores de memoria.

     // we can have only one mutable reference to a value or any number of immutable references, but not both at the same time

    // MODIFIYING STRUCTS NOT BY REFERENCE

    let mut _number = 5;
    _number += 10;

    print!("Number: {}", _number);
}
