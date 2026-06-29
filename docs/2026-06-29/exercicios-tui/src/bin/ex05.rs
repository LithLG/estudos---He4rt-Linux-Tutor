use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};

struct App {
    input: String,
    historico: Vec<String>,
    sair: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        input: String::new(),
        historico: Vec::new(),
        sair: false,
    };

    while !app.sair {
        terminal.draw(|frame| desenhar(frame, &app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(tecla) = event::read()? {
                if tecla.kind != KeyEventKind::Press {
                    continue;
                }
                match tecla.code {
                    KeyCode::Char('q') => {
                        app.sair = true;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        let comando = app.input.trim().to_string();

                        if !comando.is_empty() {
                            app.historico.push(comando);
                        }

                        app.input.clear();
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();

    Ok(())
}

fn desenhar(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let colunas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let mut texto_terminal = String::new();

    texto_terminal.push_str("Histórico:\n");

    if app.historico.is_empty() {
        texto_terminal.push_str("\n");
    } else {
        for comando in &app.historico {
            texto_terminal.push_str(&format!("$ {}\n", comando));
        }
    }

    texto_terminal.push_str("\n");
    texto_terminal.push_str(&format!("$ {}", app.input));

    let terminal_pane = Paragraph::new(texto_terminal)
        .block(
            Block::default()
                .title("Terminal Pane")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });

    let tutorial_pane =
        Paragraph::new("Lição: Navegação básica\n\nDigite no Terminal Pane.\n\nq sai do programa.")
            .block(
                Block::default()
                    .title("Tutorial Pane")
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: true });

    frame.render_widget(terminal_pane, colunas[0]);
    frame.render_widget(tutorial_pane, colunas[1]);
}
