use std::io;

fn main() {
    loop {
        let mut comando = String::new();

        println!("Digite o comando:");

        io::stdin().read_line(&mut comando).unwrap();
        let comando = comando.trim();

        if comando == "pwd" {
            println!("Passo concluído");
            break;
        } else {
            println!("Ainda não passou.");
            println!("Dica: Digite pwd\n");
        }
    }
}
