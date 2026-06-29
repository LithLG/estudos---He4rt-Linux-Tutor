use std::io;

struct TerminalPane {
    comando: String,
}

struct TutorialPane {
    titulo: String,
    instrucao: String,
    dica: String,
    mostrar_dica: bool,
}

struct GlossaryPopup {
    termo: String,
    explicacao: String,
    aberto: bool,
}

enum StatusPasso {
    Aguardando,
    Concluido,
    Falhou,
}

fn ler_linha() -> String {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).unwrap();

    entrada.trim().to_string()
}

fn mostrar_status(status: &StatusPasso) {
    match status {
        StatusPasso::Aguardando => println!("Status: aguardando comando"),
        StatusPasso::Concluido => println!("Status: passo concluído"),
        StatusPasso::Falhou => println!("Status: ainda não passou"),
    }
}

fn render_terminal(terminal: &TerminalPane) {
    println!("Terminal Pane");
    println!("$ {}", terminal.comando);
}

fn render_tutorial(tutorial: &TutorialPane, status: &StatusPasso) {
    println!("Tutorial Pane");
    println!("Lição: {}", tutorial.titulo);
    println!("Objetivo: {}", tutorial.instrucao);

    mostrar_status(status);

    if tutorial.mostrar_dica {
        println!("Dica: {}", tutorial.dica);
    }
}

fn render_popup(popup: &GlossaryPopup) {
    if popup.aberto {
        println!("Glossary Popup");
        println!("Termo: {}", popup.termo);
        println!("Explicação: {}", popup.explicacao);
    }
}

fn menu_ajuda(tutorial: &mut TutorialPane, status: &StatusPasso) {
    loop {
        println!("\nVocê errou. O que deseja fazer?");
        println!("1 - Ver dica");
        println!("2 - Ver glossário");
        println!("3 - Tentar novamente");

        let opcao = ler_linha();

        match opcao.as_str() {
            "1" => {
                tutorial.mostrar_dica = true;

                println!("\n=== DICA ===\n");
                render_tutorial(tutorial, status);
            }

            "2" => {
                let popup = GlossaryPopup {
                    termo: String::from("pwd"),
                    explicacao: String::from(
                        "pwd significa print working directory. Ele mostra o diretório atual.",
                    ),
                    aberto: true,
                };

                println!("\n=== GLOSSÁRIO ===\n");
                render_popup(&popup);
            }

            "3" => {
                println!("\nBeleza, tente novamente.");
                break;
            }

            _ => {
                println!("Opção inválida. Escolha 1, 2 ou 3.");
            }
        }
    }
}

fn main() {
    let mut tutorial = TutorialPane {
        titulo: String::from("Navegação básica"),
        instrucao: String::from("Digite pwd para descobrir onde você está."),
        dica: String::from("Digite exatamente: pwd"),
        mostrar_dica: false,
    };

    let mut status = StatusPasso::Aguardando;

    loop {
        println!("\n=== UI ATUAL ===\n");

        render_tutorial(&tutorial, &status);

        println!("\nDigite um comando:");

        let comando = ler_linha();

        let terminal = TerminalPane {
            comando: comando.clone(),
        };

        println!();
        render_terminal(&terminal);

        if comando == "pwd" {
            status = StatusPasso::Concluido;
            tutorial.mostrar_dica = false;

            println!("\n=== LIÇÃO CONCLUÍDA ===\n");
            render_tutorial(&tutorial, &status);

            println!("\nParabéns! Você concluiu a lição.");
            break;
        } else {
            status = StatusPasso::Falhou;

            menu_ajuda(&mut tutorial, &status);
        }
    }
}
