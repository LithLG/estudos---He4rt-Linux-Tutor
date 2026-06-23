use std::io;

fn main(){
    let mut comando = String::new();

    println!("Comandos:");
    io::stdin()
    .read_line(&mut comando)
    .unwrap();

    let comando = comando.trim();

    match comando {
        "iniciar" => println!("Sistema iniciado"),
        "pausar" => println!("Sistema pausado"),
        "sair" => println!("Sistema encerrado"),
        _ => println!("Comando desconhecido"),
    }
}