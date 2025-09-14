extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

    println!("Advinhe o número!");

    let numero_secreto: i32 = rand::rng().random_range(1..= 101);

    loop {

        println!("Digite o seu palpite.");

        let mut palpite: String = String::new();

        io::stdin().read_line(&mut palpite).expect("Falha ao ler entrada");

        let palpite: i32 = match palpite.trim().parse(){
            Ok(num) =>  num,
            Err(_) => {
                println!("\nDigita um número INFELIZ DAS COSTAS OCA!\n");
                continue;
            }
        };

        //println!("Número secreto é: {}", numero_secreto);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("\n *** Você acertou!");
                break;
            }
        }
    }

}