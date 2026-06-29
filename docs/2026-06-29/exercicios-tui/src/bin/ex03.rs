use std::io;

use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    terminal.draw(desenhar)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    ratatui::restore();

    Ok(())
}

fn desenhar(frame: &mut Frame) {
    let area = frame.area();

    let texto = Paragraph::new("Olá, Lucas!").block(
        Block::default()
            .title("Minha primeira TUI")
            .borders(Borders::ALL),
    );

    frame.render_widget(texto, area);
}
