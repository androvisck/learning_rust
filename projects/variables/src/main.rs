fn main() {
    let a = [1, 2, 3, 4, 5];
    for indice in a{

        let elemento = a[indice-1];

        println!("O valor do elemento é: {}", elemento);
    }
}
