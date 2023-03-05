use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //inicio
    println!("¡Adivina el número! del 1 al 15");

    //crear numero aletorio
    let secret_number: i32 = rand::thread_rng().gen_range(1..=15);

    //ver numero secreto
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Por favor, introduzca sus conjeturas. Que sea un numero");

        //se crear variable que almacenera la adivinanza del usuario
        let mut guess = String::new();

        //lee el valor ingresado por el usaurio en teclado y lo almacena en la variable guess mediante referencia usando "&", tambien tiene un expect que tiene un mensaje de error
        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la línea");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Adivinaste el numero: {guess}");

        //comparacion de resultados, el comparador cmp devuelve 3 posibles estadso que son less que es menor, greater que es mayor y Equal que signfica que el numero coicide
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("El numero es muy pequeño"),
            Ordering::Greater => println!("El numero es muy grande"),
            Ordering::Equal => {
                println!("Ganaste!!");
                break;
            }
        }
    }
}
