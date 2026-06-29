struct GlossaryPopup {
    termo: String,
    explicacao: String,
    aberto: bool,
}

fn render_popup(popup: &GlossaryPopup) {
    if popup.aberto {
        println!("Glossary Popup");
        println!("Termo: {}", popup.termo);
        println!("Explicação: {}", popup.explicacao);
    }
}

fn main() {
    let popup = GlossaryPopup {
        termo: String::from("pwd"),
        explicacao: String::from("Mostrar o diretório atual."),
        aberto: true,
    };
    render_popup(&popup);
}
