use std::io;

fn main() {
    let mut nomes: Vec<String> = Vec::new();

    for i in 1..=5 {
        let mut nome = String::new();

        println!("Digite o nome {i}:");

        io::stdin().read_line(&mut nome).unwrap();
        let nome = nome.trim().to_string();

        nomes.push(nome);
    }

    println!("\nNomes cadastrados:");

    for nome in &nomes {
        println!("{nome}");
    }
}
