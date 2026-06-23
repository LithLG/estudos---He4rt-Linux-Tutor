use std::io;

fn main(){
    let mut numero1 = String::new();
    let mut numero2 = String::new();
    let mut operacao = String::new();

    println!("Digite o primeiro número:");

    io::stdin()
    .read_line(&mut numero1)
    .unwrap();

    let numero1: f64 = numero1
    .trim()
    .parse()
    .unwrap();

    println!("Digite o tipo de operação\n
    soma = +\n
    subtração = -\n
    multiplicação = *\n
    divisão = /\n");

    io::stdin()
    .read_line(&mut operacao)
    .unwrap();

    let operacao = operacao.trim();

    println!("Digite o segundo número:");

    io::stdin()
    .read_line(&mut numero2)
    .unwrap();

    let numero2: f64 = numero2
    .trim()
    .parse()
    .unwrap();

    if operacao == "+"{
        println!("{numero1} + {numero2} = {}", numero1 + numero2);
}else if operacao == "-"{
    println!("{numero1} - {numero2} = {}", numero1 - numero2);
}else if operacao == "*"{
    println!("{numero1} * {numero2} = {}", numero1 * numero2);
}else if operacao == "/"{
    println!("{numero1} / {numero2} = {}", numero1 / numero2);
}else{
    println!("Operação inválida.");
}
}