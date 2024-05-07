use tui::{layout::Alignment,style::{Color, Style}, text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph},
};


pub fn render_contact() -> Paragraph<'static> {
    let contact = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Contact Information")]),
        Spans::from(vec![Span::raw("")]),
        // Add your contact information here
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Contact")
            .border_type(BorderType::Plain),
    );

    contact
}