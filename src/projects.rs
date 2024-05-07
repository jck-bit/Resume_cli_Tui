
use tui::{
    layout:: Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table, 
    },
};

use crate::{read_db, Project};

pub fn render_projects<'a>(project_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let project_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Projects")
        .border_type(BorderType::Plain);

    let project_list: Vec<Project> = read_db().expect("can fetch project list");
    let selected_projects = project_list
        .get(
            project_list_state
                .selected()
                .expect("there is always a selected experience"),
        )
        .expect("exists")
        .clone();

    // Create a List widget for the left side with position items
    let list = List::new(
        project_list
            .iter()
            .map(|project| ListItem::new(Spans::from(vec![Span::styled(
                project.name.clone(),
                Style::default(),
            )])))
            .collect::<Vec<_>>(),
    )
    .block(project_block.clone())
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
            "# Platform",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::raw(selected_projects.platform)),
    ]));
    
    rows.push(Row::new(vec![
        Cell::from(Span::raw("")), // Empty cell
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::styled(
            "# Stack",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]));
    rows.push(Row::new(vec![
        Cell::from(Span::raw(selected_projects.stack)),
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
for desc_line in &selected_projects.description {
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