use crate::app::{App, CurrentScreen, CurrentlyEditing};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New JSON",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for (key, value) in app.key_value_pairs.iter() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <8} : {}", key, value),
            Style::default().fg(Color::Black),
        ))));
    }

    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(chunks[1]);

    let list = List::new(list_items).style(Style::default().bg(Color::White));
    frame.render_widget(list, center_chunks[0]);

    ui_visualizer(frame, app, center_chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        if let Some(editing) = &app.currently_editing {
            match editing {
                CurrentlyEditing::Key => {
                    Span::styled("Editing JSON Key", Style::default().fg(Color::Green))
                }
                CurrentlyEditing::Value => {
                    Span::styled("Editing JSON Value", Style::default().fg(Color::LightGreen))
                }
            }
        } else {
            Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel / (Tab) to switch boxes / enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut val_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => val_block = val_block.style(active_style),
        }

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let val_text = Paragraph::new(app.value_input.clone()).block(val_block);
        frame.render_widget(val_text, popup_chunks[1]);
    }

    // Exiting
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area());

        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn ui_visualizer(frame: &mut Frame, _app: &App, area: Rect) {
    let bar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            std::iter::repeat(Constraint::Percentage(10))
                .take(10)
                .collect::<Vec<_>>(),
        )
        .split(area);

    let mut rng = rand::thread_rng();

    // Create and render each bar
    for chunk in bar_chunks.iter() {
        // Create a smaller chunk to add spacing between bars
        let bar_area = Rect::new(
            chunk.x + 1, // Add 1 for spacing on left
            chunk.y,
            chunk.width - 2, // Subtract 2 for spacing on both sides
            chunk.height,
        );

        // Generate random height (between 10% and 100% of available height)
        let height_percentage = rng.gen_range(10..=100);
        let bar_height = (bar_area.height as f32 * (height_percentage as f32 / 100.0)) as u16;

        // Calculate the y position to align the bar to the bottom
        let bar_y = bar_area.y + (bar_area.height - bar_height);

        // Create the actual bar area
        let bar_rect = Rect::new(bar_area.x, bar_y, bar_area.width, bar_height);

        // Create and render the bar
        let bar = Block::default().style(Style::default().bg(Color::Green));

        frame.render_widget(bar, bar_rect);
    }
}
