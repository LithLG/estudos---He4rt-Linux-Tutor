use std::io;

fn main() {
    let mut historico: Vec<String> = Vec::new();

    for i in 1..=3 {
        let mut comando = String::new();

        println!("Digite o comando {i}:");

        io::stdin().read_line(&mut comando).unwrap();

        let comando = comando.trim().to_string();

        historico.push(comando);
    }

    println!("\nHistórico:");

    for comando in &historico {
        println!("$ {}", comando);
    }
}
