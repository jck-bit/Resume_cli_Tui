
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
    let experience = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Projects")
        .border_type(BorderType::Plain);

    let project_list: Vec<Project> = read_db().expect("can fetch pet list");
    let items: Vec<_> = project_list
        .iter()
        .map(|project| {
            ListItem::new(Spans::from(vec![Span::styled(
                project.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_project = project_list
        .get(
            project_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(experience).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let project_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_project.id.to_string())),
        Cell::from(Span::raw(selected_project.name)),
        Cell::from(Span::raw(selected_project.category)),
        Cell::from(Span::raw(selected_project.age.to_string())),
        Cell::from(Span::raw(selected_project.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Platform",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),

        Cell::from(Span::styled(
            "Stack",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    (list, project_detail)
}