use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table},
};

use crate::{read_exerience_db, Experience};

pub fn render_experience<'a>(experience_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let experience_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Experience")
        .border_type(BorderType::Plain);

    let experience_list: Vec<Experience> =
        read_exerience_db().expect("can fetch experience list");

    let selected_experience = experience_list
        .get(
            experience_list_state
                .selected()
                .expect("there is always a selected experience"),
        )
        .expect("exists")
        .clone();

    // Create a List widget for the left side with position items
    let list = List::new(
        experience_list
            .iter()
            .map(|exp| ListItem::new(Spans::from(vec![Span::styled(
                exp.position.clone(),
                Style::default(),
            )])))
            .collect::<Vec<_>>(),
    )
    .block(experience_block.clone())
    .highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    // Create a Table widget for the right side with company, duration, and description
    let mut rows: Vec<Row> = Vec::new();
    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "# Company",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::raw(selected_experience.company)),
    ]));
    
    rows.push(Row::new(vec![
        Cell::from(Span::raw("")), // Empty cell
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "# Duration",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::raw(selected_experience.duration)),
    ]));

    rows.push(Row::new(vec![
        Cell::from(Span::raw("")), 
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "# Descrption",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]));
   
   // Assuming `description` is guaranteed to be present, you can directly iterate over it
for desc_line in &selected_experience.description {
    rows.push(Row::new(vec![
        Cell::from(Span::raw(desc_line.clone())),
    ]));
}
 
    let experience_detail = Table::new(rows)
        .header(
            Row::new(vec![
                Cell::from(Span::styled(
                    "Attribute",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
            ])
            .style(Style::default().add_modifier(Modifier::DIM)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(100)]);

    (list, experience_detail)
}
