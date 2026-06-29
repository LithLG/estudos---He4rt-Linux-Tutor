use std::io;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    terminal.draw(desenhar)?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    ratatui::restore();

    Ok(())
}

fn desenhar(frame: &mut Frame) {
    let area = frame.area();

    let colunas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let terminal_pane = Paragraph::new("$ pwd").block(
        Block::default()
            .title("Terminal Pane")
            .borders(Borders::ALL),
    );

    let tutorial_pane = Paragraph::new(
        "Lição: Navegação básica\n\nObjetivo:\nDigite pwd para descobrir onde você está.",
    )
    .block(
        Block::default()
            .title("Tutorial Pane")
            .borders(Borders::ALL),
    )
    .wrap(Wrap { trim: true });

    frame.render_widget(terminal_pane, colunas[0]);
    frame.render_widget(tutorial_pane, colunas[1]);
}
