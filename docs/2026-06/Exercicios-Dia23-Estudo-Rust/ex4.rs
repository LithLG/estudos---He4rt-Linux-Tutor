use std::io;

fn somar(a: i32,b: i32) -> i32{
    a + b
}

fn leitura() -> i32 {
    let mut entrada = String::new();

    io::stdin()
    .read_line(&mut entrada)
    .unwrap();

    entrada
    .trim()
    .parse()
    .unwrap()
}

fn main(){
    println!("Digite o primeiro número:");
    let numero1 = leitura();
    println!("Digite o segundo número:");
    let numero2 = leitura();

    let resultado = somar(numero1,numero2);

    println!("{numero1} + {numero2} = {resultado}");
}