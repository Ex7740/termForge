<<<<<<< HEAD
=======
// The actual interface of the app

>>>>>>> 627c046 (Added comments to each file for easy understanding)
use ratatui::{
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph, Sparkline, Wrap},
    style::{Color, Style},
    Frame,
};

pub fn draw(f: &mut Frame, data: &[u64], input_text: &str) {
    let size = f.area();

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(70),
            Constraint::Min(0),
        ])
        .split(size);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(20),
            Constraint::Min(0),
        ])
        .split(main_layout[0]);

    let sparkline1 = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title("1"))
        .data(data)
        .style(Style::default().fg(Color::Blue));
    
    let para2 = Paragraph::new("2")
        .block(Block::default().borders(Borders::ALL).title("2"));

    let para3 = Paragraph::new(input_text.to_string())
        .block(Block::default().borders(Borders::ALL).title("Text Input"))
        .style(Style::default().fg(Color::Reset))
        .wrap(Wrap { trim: false });

    f.render_widget(sparkline1, left_layout[0]);
    f.render_widget(para2, left_layout[1]);
    f.render_widget(para3, main_layout[1]);
}

