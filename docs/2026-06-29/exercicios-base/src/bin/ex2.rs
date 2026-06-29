struct TerminalPane {
    comando: String,
}

fn main() {
    let terminal = TerminalPane {
        comando: String::from("ls"),
    };

    println!("Terminal Pane");
    println!("$ {}", terminal.comando);
}
