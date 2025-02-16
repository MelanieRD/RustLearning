   //explanantion that i loved: https://www.youtube.com/watch?v=Mcuqzx3rBWc&t=434s
   //binary number system
    //Byte = 8 bits, if we take 8 bits we can represent 256 numbers, from 0 to 255 = 256 total values
    //for example we can represent 65 with 01000001, that happens because the binary number system is a base 2 system
    //2^8=256, 2^7=128, 2^6=64, 2^5=32, 2^4=16, 2^3=8, 2^2=4, 2^1=2, 2^0=1
    // 0          0       1        0       0      0      0      0       1        = 64 + 1 = 65
    //Para convertir ahora de un entero a un caracter, se utiliza la tabla ASCII, que es un estandar de codificacion de caracteres

fn main() {
    //Rust tiene dos tipos diferentes de Strings, el String y el &str

    //&str es un tipo de dato que es una referencia a un String, es decir, es inmutable y no puede cambiar su tamaño
    // is a fundamental data type in Rust, the string slice. It is a reference to a sequence of UTF-8 bytes stored elsewhere in memory (aplication's binary, stack or heap).
   //&str hace referencia a la dirreccion de memoria donde se encuentra el string, apunta hacia el primer byte del string, el cual segun la definicion de ut-8, es el byte que contiene la longitud del string
   //por ende el &str no tiene un tamaño fijo, ya que depende del tamaño del string al que hace referencia
   //&str trackea solo dos valores, el address (puntero) y la longitud.

   example1_str(); //string literal

    //String es un tipo de dato que si es mutable y puede cambiar su tamaño, es un uf-8 encoded, growable, heap-allocated data structure.
    //String es un tipo de dato que es un wrapper sobre un vector de bytes, es decir, es un vector de bytes que contiene caracteres utf-8
    //Los strings trackeran tres valores, El address (puntero), la longitud del string y la capacidad del string (la cantidad de bytes que tendremos para trabajar).

    example2_string(); //String en el heap, que podemos manipular

    //Podemos crear un string utilizando metodos en string literals
    let mut soy_un_string:String = "Hola, yo soy un string de verdad".to_string(); //poremos utilizar otro metodo que hace lo mismo: .to_owned()
    soy_un_string.push_str(" y me puedes modificar");
    println!("{}", soy_un_string);



    //para crear un slice string que haga referencia a un string, podemos hacerlo de la siguiente manera
    let soy_un_string_slice: &str = &soy_un_string;
    println!("{}", soy_un_string_slice);


    // ---------------------------------------------------------------- MANIPULACION DE STRINGS ----------------------------------------------------------------
    manipulation_strings();

    indexing_strings();

    //----------------------------- Iterating over strings --------------------------------
    //Iterating over strings is a bit tricky because strings are a collection of bytes, not characters.
    iterate_over_strings();

}




fn example1_str(){
let s1:&str = "Hola";  // <---- Esto es un string literal, es decir, un string slice que esta codificado en el binario de la aplicacion
println!("{}", s1);
}

fn example2_string(){
    let mut s2 = String::from("Holiwis"); // <---- Esto es un String, es decir, un vector de bytes que contiene caracteres utf-8
    s2.push_str(", soy Melanie Cool!");
    println!("{}", s2);
}


fn manipulation_strings(){

    let mut my_string = String::from("Hola ");

    //push_str() agrega un string slice al final de un string
    my_string.push_str("Melanie!");
    println!(" Pushing Melanie: {}", my_string);

    // .replace_range() reemplaza un rango de bytes en un string con otro string
    my_string.replace_range(5..12, "Mundo");//  <---- just '..' remplace the entire string, but if use numbers it will remplace the range of bytes
    println!(" Remplacing Melanie: {}", my_string);

    //concatenar strings
    let string1 = String::from("Soy ");
    let string2 = String::from("Cool!");
    let string3 = string1 + &string2; // <---- string1 se mueve a string3, por lo que no podemos usar string1 despues de esta linea
    //También podemos concatenar strings de la siguiente manera: string1 = string1 + &string2;

    //println!("String1: {}", string1);  <---- Esto da error porque string1 se movio a string3
    println!("String2: {}", string2);
    println!("Concatenando strings: {}", string3);

    //Otra frma es format! macro que permite concatenar strings sin mover la propiedad format!() OJO ESTO ES MENOS EFICIENTE Porque esta creando un nuevo string en memoria
    let string4 = String::from("Soy ");
    let string5 = String::from("Cool!");
    let string6 = String::from(" y muy feliz de poder programar :D");
    let string7 = format!("{}{}{}", string4, string5, string6); // <---- string4, string5 y string6 no se mueven a string7, esto se convierte en uns string type
    let _string8 = concat!("Hola" ," No se", " que mas escribir"); // <---- concatenar literales de strings y conseguir un string slice
    println!("Concatenando strings con format!: {}", string7);
    println!("Concatenando strings con concat!: {}", string7);

    //concat Arrays of Strings
    let _array_of_strings = ["Hola", "Melanie", "Cool!"].concat();


}

fn indexing_strings(){
    let string_emojies:&str = "❤️😝🤑😫👿💀";
    println!("{}", string_emojies);

    //remember that rust use bytes to index strings, so if we want to get the first emoji we need to get the first 4 bytes
    //let specific_emoji = &string_emojies[0]; <---- Esto da error porque no podemos indexar un string por bytes, ya que los emojis son de 4 bytes
    // but we can create a slice of the string to get the first emoji using &string_emojies[0..3] because the first emoji is 4 bytes
    let specific_emoji = &string_emojies[0..3];
    println!("The first emoji is: {}", specific_emoji);

    //--------------------------------- Getting a specific character from a string ---------------------------------
   if let Some(c) = string_emojies.chars().nth(2){
     println!("{}",c); // <---- Esto nos da el segundo emoji de la cadena de emojis
    }

    let char_byte = string_emojies.as_bytes()[3] as char; // <---- No funciona con uft-8 solo con ASCII por eso no nos da el emoji sino que otro simbolo ASCII
    println!("as bytes: {}", char_byte); // <---- Esto nos da el byte del primer emoji de la cadena de emojis


}

fn iterate_over_strings() {
let a = "aaa".to_string();
let b = "eee".to_string();

//iterate using .bytes method (it will give you the bytes (integer) of the string)
for byte in a.bytes() {
    println!("{}", byte);
}

//iterate using .chars method (it will give you the characters of the string)
for charletter in b.chars() {
    println!("{}", charletter);
}


//iterate using .graphemes method (it will give you the graphemes of the string)

//let mut new_string =String::new(); // <---- Esto es un string vacio, ES mejor crear el string con una capacidad para que no ocupe mucho espacio en memoria;
let mut new_string = String::with_capacity(50);
let mut counter = 0;
loop{

//println!("Hola");


    if let Some(c) = a.chars().nth(counter){
    println!("{}",c); 
    new_string.push(c);
   }

   if let Some(c) = b.chars().nth(counter){
    println!("{}",c); 
    new_string.push(c);
   }

   
//breaking the loop
    if counter == 10{
        break;
    }
    counter += 1;
}
println!("{}", new_string);


}
   
