use std::io;

fn main (){
    let mut idade = String::new();
    println!("Digite sua idade:");

    io::stdin()
    .read_line(&mut idade)
    .expect("Erro ao lera a idade");

    let idade: i32 = idade
    .trim()
    .parse()
    .expect("Digite um número válido");

    if idade >= 18 {
        println!("Você é maior de idade.");
    }
    else{
        println!("Você é menor de idade.");
    }
}