use tui::{layout::Alignment,style::{Color, Style}, text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph},
};


pub fn render_contact() -> Paragraph<'static> {

    let email_title = Span::styled(
        "Email: ",
        Style::default().fg(Color::Yellow), 
    );
    let email_address = Span::styled(
        "jckkinyanjui@gmail.com",
        Style::default().fg(Color::Blue)
    );

    let linkedin_title = Span::styled(
        "LinkedIn: ",
        Style::default().fg(Color::Yellow), 
    );
    let linkedin_address = Span::styled(
        "Jackkinyanjui",
        Style::default().fg(Color::Blue)
    );

    let twitter_title = Span::styled(
        "Twitter: ",
        Style::default().fg(Color::Yellow), 
    );
    let twitter_handle = Span::styled(
        "https://twitter.com/KinyanjuiJack1",
        Style::default().fg(Color::Blue),
    );

    let phone_title = Span::styled(
        "Phone No: ",
        Style::default().fg(Color::Yellow), 
    );
    let phone_number = Span::styled(
        "+254790407966",
        Style::default().fg(Color::Blue),
    );

    // Construct the paragraph with styled spans
    let contact = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Contact Information")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![email_title, email_address]),
        Spans::from(vec![linkedin_title, linkedin_address]), 
        Spans::from(vec![phone_title, phone_number]),
        Spans::from(vec![twitter_title, twitter_handle]), 

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
