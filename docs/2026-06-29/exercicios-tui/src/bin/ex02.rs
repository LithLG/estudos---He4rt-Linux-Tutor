use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut input = String::new();

    println!("Digite algo.");
    println!("Enter envia o comando.");
    println!("Backspace apaga.");
    println!("q sai do programa.");

    loop {
        if let Event::Key(tecla) = event::read()? {
            if tecla.kind != KeyEventKind::Press {
                continue;
            }
            match tecla.code {
                KeyCode::Char('q') => {
                    println!("\nSaindo...");
                    break;
                }

                KeyCode::Char(c) => {
                    input.push(c);
                    println!("\nInput atual: {}", input);
                }

                KeyCode::Backspace => {
                    input.pop();
                    println!("\nInput atual: {}", input);
                }

                KeyCode::Enter => {
                    println!("\nComando enviado: {}", input);
                    input.clear();
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
