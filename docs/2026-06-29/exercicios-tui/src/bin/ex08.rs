use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

enum StatusPasso {
    Aguardando,
    Concluido,
    Falhou,
}

#[derive(PartialEq)]
enum Painel {
    Terminal,
    Tutorial,
}

struct App {
    input: String,
    historico: Vec<String>,
    status: StatusPasso,
    mostrar_dica: bool,
    foco: Painel,
    glossario_aberto: bool,
    sair: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        input: String::new(),
        historico: Vec::new(),
        status: StatusPasso::Aguardando,
        mostrar_dica: false,
        foco: Painel::Terminal,
        glossario_aberto: false,
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

                    KeyCode::Esc => {
                        if app.glossario_aberto {
                            app.glossario_aberto = false;
                        } else {
                            app.sair = true;
                        }
                    }

                    KeyCode::Char('?') => {
                        app.glossario_aberto = true;
                    }

                    KeyCode::Tab => {
                        if !app.glossario_aberto {
                            app.foco = match app.foco {
                                Painel::Terminal => Painel::Tutorial,
                                Painel::Tutorial => Painel::Terminal,
                            };
                        }
                    }

                    KeyCode::Char('h') => {
                        if !app.glossario_aberto {
                            app.mostrar_dica = !app.mostrar_dica;
                        }
                    }

                    KeyCode::Char(c) => {
                        if app.foco == Painel::Terminal && !app.glossario_aberto {
                            app.input.push(c);
                        }
                    }

                    KeyCode::Backspace => {
                        if app.foco == Painel::Terminal && !app.glossario_aberto {
                            app.input.pop();
                        }
                    }

                    KeyCode::Enter => {
                        if app.foco == Painel::Terminal && !app.glossario_aberto {
                            let comando = app.input.trim().to_string();

                            if !comando.is_empty() {
                                app.historico.push(comando.clone());

                                if comando == "pwd" {
                                    app.status = StatusPasso::Concluido;
                                    app.mostrar_dica = false;
                                } else {
                                    app.status = StatusPasso::Falhou;
                                }
                            }

                            app.input.clear();
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    ratatui::restore();

    Ok(())
}

fn texto_status(status: &StatusPasso) -> &'static str {
    match status {
        StatusPasso::Aguardando => "Status: aguardando comando",
        StatusPasso::Concluido => "Status: Passo concluído. Parabéns!",
        StatusPasso::Falhou => "Status: ainda não passou",
    }
}

fn desenhar(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let colunas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    desenhar_terminal(frame, colunas[0], app);
    desenhar_tutorial(frame, colunas[1], app);

    if app.glossario_aberto {
        desenhar_glossario(frame, area);
    }
}

fn desenhar_terminal(frame: &mut Frame, area: Rect, app: &App) {
    let mut texto = String::new();

    texto.push_str("Histórico:\n");

    if app.historico.is_empty() {
        texto.push_str("\n");
    } else {
        for comando in &app.historico {
            texto.push_str(&format!("$ {}\n", comando));
        }
    }

    texto.push_str("\n");
    texto.push_str(&format!("$ {}", app.input));

    let titulo = if app.foco == Painel::Terminal {
        "Terminal Pane [FOCO]"
    } else {
        "Terminal Pane"
    };

    let borda = if app.foco == Painel::Terminal {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let pane = Paragraph::new(texto)
        .block(
            Block::default()
                .title(titulo)
                .borders(Borders::ALL)
                .border_style(borda),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(pane, area);
}

fn desenhar_tutorial(frame: &mut Frame, area: Rect, app: &App) {
    let dica = if app.mostrar_dica {
        "Dica: o comando correto é pwd."
    } else {
        "Pressione h para ver a dica."
    };

    let texto = format!(
        "Lição: Navegação básica\n\nObjetivo:\nDigite pwd para descobrir onde você está.\n\n{}\n\n{}\n\nPressione ? para abrir o glossário.",
        texto_status(&app.status),
        dica
    );

    let titulo = if app.foco == Painel::Tutorial {
        "Tutorial Pane [FOCO]"
    } else {
        "Tutorial Pane"
    };

    let borda = if app.foco == Painel::Tutorial {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let pane = Paragraph::new(texto)
        .block(
            Block::default()
                .title(titulo)
                .borders(Borders::ALL)
                .border_style(borda),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(pane, area);
}

fn desenhar_glossario(frame: &mut Frame, area: Rect) {
    let popup_area = area_centralizada(60, 50, area);

    frame.render_widget(Clear, popup_area);

    let texto = "Termo: pwd\n\n\
Significa print working directory.\n\n\
Mostra o diretório atual.\n\n\
Pressione Esc para fechar.";

    let popup = Paragraph::new(texto)
        .block(
            Block::default()
                .title("Glossary Popup")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(popup, popup_area);
}

fn area_centralizada(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
