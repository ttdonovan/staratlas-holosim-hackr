use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Table, Row, Cell},
    Frame, Terminal,
};
use std::{io, time::Duration};
mod app;
mod data;

use app::{App, CurrentScreen};

fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt::init();

    // Load game data
    let game_data_path = if std::path::Path::new("gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron").exists() {
        "gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron"
    } else {
        "../../gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron"
    };
    let mut app = App::new(game_data_path)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.current_screen {
                        CurrentScreen::Main => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('1') => app.current_screen = CurrentScreen::Ships,
                            KeyCode::Char('2') => app.current_screen = CurrentScreen::Planets,
                            KeyCode::Char('3') => app.current_screen = CurrentScreen::Sectors,
                            KeyCode::Char('4') => app.current_screen = CurrentScreen::Starbases,
                            _ => {}
                        },
                        _ => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.current_screen = CurrentScreen::Main,
                            KeyCode::Up => app.previous(),
                            KeyCode::Down => app.next(),
                            KeyCode::PageUp => app.page_up(),
                            KeyCode::PageDown => app.page_down(),
                            _ => {}
                        },
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Main => draw_main_menu(f, app),
        CurrentScreen::Ships => draw_ships_screen(f, app),
        CurrentScreen::Planets => draw_planets_screen(f, app),
        CurrentScreen::Sectors => draw_sectors_screen(f, app),
        CurrentScreen::Starbases => draw_starbases_screen(f, app),
    }
}

fn draw_main_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("Star Atlas Holosim Data Viewer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            Span::styled("1. ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Ships ({} total)", app.game_data.ships.len())),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("2. ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Planets ({} total)", app.game_data.planets.len())),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("3. ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Sectors ({} total)", app.game_data.sectors.len())),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("4. ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Starbases ({} total)", app.game_data.starbases.len())),
        ])),
    ];

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .style(Style::default().fg(Color::White));
    f.render_widget(menu, chunks[1]);

    let instructions = Paragraph::new("Press number to select, 'q' to quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

fn draw_ships_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let title = Paragraph::new(format!("Ships - Page {} of {}", app.current_page() + 1, app.total_pages()))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Ship table
    let header = Row::new(vec!["Name", "Size Class", "Cargo Cap", "Required Crew", "Fuel Cap"])
        .style(Style::default().fg(Color::Yellow))
        .height(1)
        .bottom_margin(1);

    let items = app.get_current_ships();
    let rows: Vec<Row> = items
        .iter()
        .map(|ship| {
            Row::new(vec![
                Cell::from(ship.name.clone()),
                Cell::from(format!("{}", ship.size_class)),
                Cell::from(format!("{}", ship.stats.cargo_stats.cargo_capacity)),
                Cell::from(format!("{}", ship.stats.misc_stats.required_crew)),
                Cell::from(format!("{}", ship.stats.cargo_stats.fuel_capacity)),
            ])
        })
        .collect();

    let table = Table::new(rows, &[
        Constraint::Percentage(30),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(table, chunks[1]);

    let instructions = Paragraph::new("↑/↓: Navigate | PgUp/PgDn: Page | ESC/q: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

fn draw_planets_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let title = Paragraph::new(format!("Planets - Page {} of {}", app.current_page() + 1, app.total_pages()))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let header = Row::new(vec!["Name", "Sector", "Type", "Size", "Resources", "Health"])
        .style(Style::default().fg(Color::Yellow))
        .height(1)
        .bottom_margin(1);

    let items = app.get_current_planets();
    let rows: Vec<Row> = items
        .iter()
        .map(|planet| {
            Row::new(vec![
                Cell::from(planet.name.clone()),
                Cell::from(format!("({}, {})", planet.sector[0], planet.sector[1])),
                Cell::from(format!("{}", planet.planet_type)),
                Cell::from(format!("{}", planet.size)),
                Cell::from(format!("{}", planet.num_resources)),
                Cell::from(format!("{}/{}", planet.current_health, planet.max_hp)),
            ])
        })
        .collect();

    let table = Table::new(rows, &[
        Constraint::Percentage(25),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(table, chunks[1]);

    let instructions = Paragraph::new("↑/↓: Navigate | PgUp/PgDn: Page | ESC/q: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

fn draw_sectors_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let title = Paragraph::new(format!("Sectors - Page {} of {}", app.current_page() + 1, app.total_pages()))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let header = Row::new(vec!["Name", "Coordinates", "Stars", "Planets", "Connections"])
        .style(Style::default().fg(Color::Yellow))
        .height(1)
        .bottom_margin(1);

    let items = app.get_current_sectors();
    let rows: Vec<Row> = items
        .iter()
        .map(|sector| {
            Row::new(vec![
                Cell::from(sector.name.clone()),
                Cell::from(format!("({}, {})", sector.coordinates.0, sector.coordinates.1)),
                Cell::from(format!("{}", sector.num_stars)),
                Cell::from(format!("{}", sector.num_planets)),
                Cell::from(format!("{}", sector.num_connections)),
            ])
        })
        .collect();

    let table = Table::new(rows, &[
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(table, chunks[1]);

    let instructions = Paragraph::new("↑/↓: Navigate | PgUp/PgDn: Page | ESC/q: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

fn draw_starbases_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let title = Paragraph::new(format!("Starbases - Page {} of {}", app.current_page() + 1, app.total_pages()))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let header = Row::new(vec!["Name", "Sector", "Faction", "Level", "HP/SP"])
        .style(Style::default().fg(Color::Yellow))
        .height(1)
        .bottom_margin(1);

    let items = app.get_current_starbases();
    let rows: Vec<Row> = items
        .iter()
        .map(|starbase| {
            Row::new(vec![
                Cell::from(starbase.name.clone()),
                Cell::from(format!("({}, {})", starbase.sector.0, starbase.sector.1)),
                Cell::from(format!("{}", starbase.faction)),
                Cell::from(format!("{}", starbase.level)),
                Cell::from(format!("{}/{}", starbase.hp, starbase.sp)),
            ])
        })
        .collect();

    let table = Table::new(rows, &[
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(table, chunks[1]);

    let instructions = Paragraph::new("↑/↓: Navigate | PgUp/PgDn: Page | ESC/q: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}