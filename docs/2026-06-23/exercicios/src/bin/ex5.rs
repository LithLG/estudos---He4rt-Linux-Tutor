use std::io;

fn main() {
    let mut nome = String::new();

    println!("Digite teu nome:");

    io::stdin().read_line(&mut nome).unwrap();

    let nome = nome.trim();

    println!("Olá, {nome}!");
}
