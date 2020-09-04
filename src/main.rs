// Importa biblioteca std o pacote io | Rand e cmp
use rand::Rng;
use std::io;
use std::cmp::Ordering;
// função principal de inicio como C
fn main() {
    println!("Adivinhe um número!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Por favor você insira um numero.");
    // quando tem mut ele é mutavel
    let mut guess = String::new();

    // :: seria buscar o atributo ou metodo dentro
    // stdin retorona uma instancia de Stdin
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // tipa para unsigned 32-bit number 
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Muito abaxio!"),
        Ordering::Greater => println!("muito acima!"),
        Ordering::Equal => println!("Você ganhou!"),
    }
}