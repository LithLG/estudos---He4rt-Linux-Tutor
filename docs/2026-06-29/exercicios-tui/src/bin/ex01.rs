use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    println!("Pressione teclas.");
    println!("Pressione q para sair.");

    loop {
        if let Event::Key(tecla) = event::read()? {
            if tecla.kind != KeyEventKind::Press {
                continue;
            }
            match tecla.code {
                KeyCode::Char('q') => {
                    println!("Pressionou q. Saindo...");
                    break;
                }
                KeyCode::Char(c) => {
                    println!("Você apertou: {}", c);
                }
                KeyCode::Enter => {
                    println!("Você apertou Enter");
                }
                KeyCode::Esc => {
                    println!("Você apertou Esc");
                }
                _ => {
                    println!("Você apertou outra tecla");
                }
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
