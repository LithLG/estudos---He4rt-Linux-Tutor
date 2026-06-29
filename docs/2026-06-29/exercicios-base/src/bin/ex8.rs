use std::io;

fn main() {
    let instrucao = "Digite pwd para verificar onde você está.";
    let dica = "O comando é pwd.";

    println!("Tutorial Pane");
    println!("Objetivo: {}", instrucao);

    loop {
        let mut comando = String::new();

        println!("\nDigite sua resposta:");

        io::stdin().read_line(&mut comando).unwrap();

        let comando = comando.trim();

        if comando == "pwd" {
            println!("Passo concluído!");
            break;
        } else {
            println!("Comando errado.");
        }

        let mut resposta = String::new();

        println!("Deseja ver a dica? s/n");

        io::stdin().read_line(&mut resposta).unwrap();

        let resposta = resposta.trim();

        if resposta == "s" {
            println!("Dica: {}", dica);
        } else {
            println!("Dica ocultada, tente novamente.");
        }
    }
}
