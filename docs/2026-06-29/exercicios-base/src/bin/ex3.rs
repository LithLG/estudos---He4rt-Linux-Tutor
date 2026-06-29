struct TerminalPane {
    comando: String,
}

fn render_terminal(terminal: &TerminalPane) {
    println!("Terminal Pane");
    println!("$ {}", terminal.comando);
}

fn main() {
    let terminal = TerminalPane {
        comando: String::from("pwd"),
    };

    render_terminal(&terminal);
}
