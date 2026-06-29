struct TutorialPane {
    titulo: String,
    instrucao: String,
    dica: String,
    mostrar_dica: bool,
}

fn render_tutorial(tutorial: &TutorialPane) {
    println!("Tutorial Pane");
    println!("Lição: {}", tutorial.titulo);
    println!("Objetivo: {}", tutorial.instrucao);

    if tutorial.mostrar_dica {
        println!("Dica: {}", tutorial.dica)
    }
}

fn main() {
    let tutorial = TutorialPane {
        titulo: String::from("Navegação básica"),
        instrucao: String::from("Use pwd para verificar onde você está."),
        dica: String::from("Digite: pwd"),
        mostrar_dica: true,
    };
    render_tutorial(&tutorial);
}
