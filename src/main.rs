mod experience;
mod projects;

use chrono::{DateTime, Utc};
use experience:: render_experience;
use projects:: render_projects;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, ListState, Paragraph, Tabs,
    },
    Terminal,
};

const DB_PATH: &str = "./data/db.json";
const EXPERIENCE_DB_PATH : &str= "./data/experience.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone)]
struct Project {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
struct  Experience {
    id: usize,
    position: String,
    company:String,
    duration:String,
    description:Vec<String>
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Projects,
    Experience,
    Contact,
//    Quit,
}


impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Projects => 1,
            MenuItem::Experience => 2,
            MenuItem::Contact => 3,
          //  MenuItem::Quit => 4,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Projects", "Experience", "Contact", "Quit"];
    let mut active_menu_item = MenuItem::Home;
    let mut project_list_state = ListState::default();
    project_list_state.select(Some(0));

    let  mut experience_list_state   = ListState::default();
    experience_list_state.select(Some(0));


    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new("Jack Kinyanjui 2024 - all rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Projects => {
                    let project_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_projects(&project_list_state);
                    rect.render_stateful_widget(left, project_chunks[0], &mut project_list_state);
                    rect.render_widget(right, project_chunks[1]);
                }
                
                MenuItem::Experience => {
                    
                    let experience_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (list, detail) = render_experience(&experience_list_state);
                    rect.render_stateful_widget(list, experience_chunks[0], &mut experience_list_state);
                    rect.render_widget(detail, experience_chunks[1]);
                }
                
                MenuItem::Contact => rect.render_widget(render_contact(), chunks[1]),
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('p') => active_menu_item = MenuItem::Projects,
                KeyCode::Char('e') => active_menu_item = MenuItem::Experience,
                KeyCode::Char('c') => active_menu_item = MenuItem::Contact,  
               
                KeyCode::Down => {
                    if let Some(selected) = project_list_state.selected() {
                        let amount_pets = read_db().expect("can fetch pet list").len();
                        if selected >= amount_pets - 1 {
                            project_list_state.select(Some(0));
                        } else {
                            project_list_state.select(Some(selected + 1));
                        }
                    }
                    if let Some(selected) = experience_list_state.selected() {
                            let amount_experiences = read_exerience_db().expect("can fetch experience list").len();
                            if selected > 0 {
                                experience_list_state.select(Some(selected - 1));
                            } else {
                                experience_list_state.select(Some(amount_experiences - 1));
                            }
                        }
                }
                KeyCode::Up => {
                    if let Some(selected) = project_list_state.selected() {
                        let amount_pets = read_db().expect("can fetch pet list").len();
                        if selected > 0 {
                            project_list_state.select(Some(selected - 1));
                        } else {
                            project_list_state.select(Some(amount_pets - 1));
                        }
                    }
                    if let Some(selected) = experience_list_state.selected() {
                        let amount_experiences = read_exerience_db().expect("can fetch experience list").len();
                        if selected >= amount_experiences - 1 {
                            experience_list_state.select(Some(0));
                        } else {
                            experience_list_state.select(Some(selected + 1));
                        }

                    }
                }

                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to my")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Resume -CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to view Projects, 'e' to view Experience,'c' to view Contacts  press 'q' to quit the terminal and 'h' to go back home.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_contact() -> Paragraph<'static> {
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


fn read_db() -> Result<Vec<Project>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Project> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn read_exerience_db() -> Result<Vec<Experience>, Error> {
    let db_content = fs::read_to_string(EXPERIENCE_DB_PATH)?;
    let parsed:Vec<Experience> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}