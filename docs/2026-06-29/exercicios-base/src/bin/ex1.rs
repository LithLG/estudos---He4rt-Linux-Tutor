fn mostrar_terminal(comando: &str) {
    println!("┌ Terminal Pane ┐");
    println!("$ {}", comando);
}

fn main() {
    mostrar_terminal("pwd");
}
