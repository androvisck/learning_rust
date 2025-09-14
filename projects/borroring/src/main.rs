// fn main() {
//     let s1 = String::from("texto");

//     let tamanho = calcula_tamanho(&s1);

//     println!("\n\tO tamanho de '{}' é {}.\n", s1, tamanho);
// }

// fn calcula_tamanho(s: &String) -> usize {
//     s.len()
// }


// fn main() {
//     let mut s = String::from("texto");

//     {
//         let r1 = &mut s;
//         println!("r1 = {}", r1);

//     } // aqui r1 sai de escopo, então já podemos criar uma nova referência sem
//     // problema nenhum.

//     let r2 = &mut s;

//     r2.push_str(" string");

//     println!("r2 = {}", r2);
// }

// fn main(){
//     let mut s = String::from("texto");
//     {
//     let r1 = &s; // sem problema
//     let r2 = &s; // sem problema
//     println!("r1 = {}", r1);
//     println!("r2 = {}", r2);
//     }

//     let r3 = &mut s; // PROBLEMA GRANDE
//     r3.push_str(" string");
//     println!("r3 = {}", r3);
// }

// fn main() {
//     let referencia_para_o_nada = soltar();
// }

// fn soltar() -> String {
//     let s = String::from("texto");

//     s
// }



// fn main() {

//     fn primeira_palavra(s: &String) -> usize {
//         let bytes = s.as_bytes();

//         for (i, &item) in bytes.iter().enumerate() {
//             if item == b' ' {
//                 println!("espaço-posição: {}", i);
//             } else {
//                 let one = s.len() as usize;
//                 println!("o comprimento da frase é: {}", one);
//             }
//         }

//         s.len()
//     }

//     let mut s: String = String::from("texto longo de camões");
//     println!("{}", s);
//     let palavra: usize = primeira_palavra(&s); // palavra vai ter o valor 5.
//     println!("{}", palavra);
//     s.clear(); // Isso esvazia a String, deixando ela igual a "".
//     println!("{}", palavra);
//     println!("nada {}", s);
// }



// fn main(){
//     let exemplo = String::from("paralelepipedo");

//     let slice1 = &exemplo[0..3];
//     let slice2 = &exemplo[0..6];
//     let slice3 = &exemplo[6..9];
//     let slice4 = &exemplo[6..];
//     let slice5 = &exemplo[..9];

//     println!("{}, {}, {}, {}, {}",slice1,slice2,slice3,slice4,slice5)
// }


#![allow(unused)]
fn main() {
    fn primeira_palavra(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    let mut s: String = String::from("texto longo de camões");
    println!("{}", s);
    let palavra = primeira_palavra(&s); // palavra vai ter o valor 5.
    println!("{}", palavra);
}
