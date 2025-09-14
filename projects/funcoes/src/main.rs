fn main() {
    let (x,y) = soma_um(5);

    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

fn soma_um(x: i32) -> (i32, i32){
    let y = x + 1;
    let z = y + 2;
    return (y, z)
}

