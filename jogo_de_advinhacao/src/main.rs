use std::io;

fn main() {
    println!("Advinha o número!");
    println!("Por favor, dê o seu palpite");

    let mut palpite = String::new();

    io::stdin()
    .read_line(&mut palpite)
    .expect("Falha ao ler essa linha");

    println!("Seu palpite: {}", palpite);
}
