use lopdf::Document;
use std::io::Write;
use std::time::{Duration, Instant};
use std::path::Path;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{poll, read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::text::{Line, Span};
use serde::{Serialize, Deserialize};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Clear, Paragraph, Wrap},
    Terminal,
};


#[derive(Serialize, Deserialize)]
struct SaveState {
    pdf_path: String,
    current_batch: usize,
    personal_best: u64,
    speed: u64,
    accuracy: u64,
}

fn get_sentences(path_to_the_file: &str) -> Result<Vec<String>, String> {
    let document = Document::load(path_to_the_file)
        .map_err(|error| format!("Could not open the PDF: {error}"))?;
    let page_numbers: Vec<u32> = document.get_pages().keys().copied().collect();
    if page_numbers.is_empty() {
        return Err("The PDF does not contain any pages.".to_string());
    }

    let extract_raw_page = document
        .extract_text(&page_numbers)
        .map_err(|error| format!("Could not extract text from the PDF: {error}"))?;
    let mut sentence: Vec<String> = Vec::new();
    
    for line in extract_raw_page.lines() {
        let cleaned_line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if !cleaned_line.is_empty() {
            sentence.push(cleaned_line);
        }
    }
    if sentence.is_empty() {
        return Err("The PDF does not contain any selectable text.".to_string());
    }
    Ok(sentence)
}

const CHARS_PER_WORD: f64 = 5.0;
const MAX_REASONABLE_WPM: u64 = 400;

fn accuracy_percent(typed_chars: &[char], target: &str) -> u64 {
    if typed_chars.is_empty() {
        return 0;
    }
    let correct_count = typed_chars
        .iter()
        .zip(target.chars())
        .filter(|(typed_char, target_char)| **typed_char == *target_char)
        .count();
    ((correct_count as f64 / typed_chars.len() as f64) * 100.0)
        .round()
        .clamp(0.0, 100.0) as u64
}

fn words_per_minute(typed_chars: &[char], elapsed: Duration) -> u64 {
    let seconds = elapsed.as_secs_f64();
    if typed_chars.is_empty() || seconds <= 0.0 {
        return 0;
    }
    let words = typed_chars.len() as f64 / CHARS_PER_WORD;
    let minutes = seconds / 60.0;
    let wpm = words / minutes;
    if !wpm.is_finite() {
        return 0;
    }
    wpm.round().clamp(0.0, MAX_REASONABLE_WPM as f64) as u64
}

fn validate_pdf_path(path: &str) -> Result<(), String> {
    let trimmed_path = path.trim();
    if trimmed_path.is_empty() {
        return Err("Enter a PDF file path to begin.".to_string());
    }
    if !trimmed_path.to_ascii_lowercase().ends_with(".pdf") {
        return Err("The selected file must have a .pdf extension.".to_string());
    }
    if !Path::new(trimmed_path).is_file() {
        return Err("That file does not exist. Check the path and try again.".to_string());
    }
    Ok(())
}

fn select_pdf_path(initial_path: Option<&str>) -> Option<String> {
    execute!(std::io::stdout(), EnterAlternateScreen).ok()?;
    if enable_raw_mode().is_err() {
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
        return None;
    }
    let result = (|| {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).ok()?;
        let mut path = initial_path.unwrap_or_default().to_string();
        let mut error_message = String::new();

        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                let vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(2),
                        Constraint::Length(7),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ])
                    .split(area);

                let logo = Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled(
                            "  ESCAPE",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            "TYPO",
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(""),
                ])
                .alignment(Alignment::Center);
                frame.render_widget(logo, vertical[1]);

                let prompt = Paragraph::new("Choose a PDF to practise")
                    .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Center);
                frame.render_widget(prompt, vertical[2]);

                let input_width = vertical[3].width.saturating_sub(4);
                let input = Paragraph::new(format!("> {path}"))
                    .style(Style::default().fg(Color::Yellow))
                    .block(ratatui::widgets::Block::bordered().title(" PDF path "));
                let input_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Min(0),
                        Constraint::Length(input_width.min(vertical[3].width)),
                        Constraint::Min(0),
                    ])
                    .split(vertical[3])[1];
                frame.render_widget(input, input_area);

                let help = if error_message.is_empty() {
                    "Enter  continue    Esc  quit"
                } else {
                    &error_message
                };
                frame.render_widget(
                    Paragraph::new(help)
                        .style(Style::default().fg(if error_message.is_empty() {
                            Color::DarkGray
                        } else {
                            Color::Red
                        }))
                        .alignment(Alignment::Center)
                        .wrap(Wrap { trim: true }),
                    vertical[4],
                );
            }).ok()?;

            if poll(Duration::from_millis(100)).ok()? {
                if let Event::Key(key_event) = read().ok()? {
                    match key_event.code {
                        KeyCode::Esc => return None,
                        KeyCode::Enter => match validate_pdf_path(&path) {
                            Ok(()) => match get_sentences(path.trim()) {
                                Ok(_) => return Some(path.trim().to_string()),
                                Err(message) => error_message = message,
                            },
                            Err(message) => error_message = message,
                        },
                        KeyCode::Backspace => {
                            path.pop();
                            error_message.clear();
                        }
                        KeyCode::Char(character) => {
                            path.push(character);
                            error_message.clear();
                        }
                        _ => {}
                    }
                }
            }
        }
    })();
    let _ = disable_raw_mode();
    let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
    result
}


//get the TUI running to get the user screen he wants 
fn start_TUI(
    sentences: Vec<String>,
    pdf_path: String,
    current_batch: usize,
    personal_best: u64,
    saved_speed: u64,
    saved_accuracy: u64,
){
    execute!(std::io::stdout(), EnterAlternateScreen).expect("Failed to enter alternate screen");
    enable_raw_mode().expect("Failed to enable raw mode");
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");
    let mut face_frame = 0usize;
    let mut typed_chars: Vec<char> = Vec::new();
    let mut batch_start = current_batch.min(sentences.len());
    let mut personal_best = personal_best;
    let mut latest_speed = saved_speed;
    let mut latest_accuracy = saved_accuracy;
    let ghost_speed = personal_best;
    let mut batch_start_time = Instant::now();
    let mut ghost_batch_start = Instant::now();
    let face_frames = [
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᵔ˶)",
        "(ꈍᴗꈍ)",  
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)"
    ];


    loop{
        if batch_start >= sentences.len() {
            disable_raw_mode().expect("Failed to disable raw mode");
            execute!(std::io::stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");
            break;
        }
        let batch_end = (batch_start + 4).min(sentences.len());
        let text = sentences[batch_start..batch_end].join("\n");

        let accuracy = if typed_chars.is_empty() {
            latest_accuracy
        } else {
            accuracy_percent(&typed_chars, &text)
        };
        let batch_elapsed = batch_start_time.elapsed();
        let speed = if typed_chars.is_empty() || batch_elapsed.as_secs_f64() < 1.0 {
            latest_speed
        } else {
            words_per_minute(&typed_chars, batch_elapsed)
        };
        latest_accuracy = accuracy;
        latest_speed = speed;
        let text_character_count = text.chars().filter(|character| *character != '\n').count();
        let ghost_characters_per_second = ghost_speed as f64 * 5.0 / 60.0;
        let ghost_position = if text_character_count == 0 || ghost_characters_per_second == 0.0 {
            None
        } else {
            Some(
                ((ghost_batch_start.elapsed().as_secs_f64() * ghost_characters_per_second)
                    as usize)
                    .min(text_character_count - 1),
            )
        };

        terminal.draw(|f| {
            let screen = Layout::default()
                .direction(Direction::Vertical)
                    .constraints([
                    Constraint::Length(2),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.size());

            let header = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(screen[0]);

            let white = Style::default().fg(Color::White);
            let bold_white = white.add_modifier(Modifier::BOLD);



            f.render_widget(
                Paragraph::new("00:00")
                    .style(white)
                    .alignment(Alignment::Right),
                header[2],
            );

            let mut colored_captured_lines = Vec::new();
            let mut character_index = 0;
            let mut visible_character_index = 0;
            // Make the letters change color based on whether the character is correct or wrong.
            for target_line in text.split('\n') {
                let mut colored_captured_text = Vec::new();
                for target_char in target_line.chars() {
                    let is_ghost_position = ghost_position == Some(visible_character_index);
                    let character_style = if let Some(typed_char) = typed_chars.get(character_index) {
                        if *typed_char == target_char {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::Red)
                        }
                    } else {
                        Style::default().fg(Color::White)
                    };
                    let character_style = if is_ghost_position {
                        character_style.bg(Color::Blue)
                    } else {
                        character_style
                    };

                    colored_captured_text.push(Span::styled(target_char.to_string(), character_style));
                    character_index += 1;
                    visible_character_index += 1;
                }
                colored_captured_lines.push(Line::from(colored_captured_text));
                character_index += 1;
            }

            f.render_widget(
                Paragraph::new(format!(
                    "WPM  {}   ACC  {}%   PB  {}",
                    speed, accuracy, personal_best
                ))
                    .style(white)
                    .alignment(Alignment::Center),
                header[1],
            );

            let display_height = text.lines().count().max(1) as u16;
            let centered_text = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(display_height),
                    Constraint::Min(0),
                ])
                .split(screen[1]);

            f.render_widget(Clear, screen[1]);
            f.render_widget(
                Paragraph::new(colored_captured_lines)
                    .style(bold_white)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false }),
                centered_text[1],
            );

            let footer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(16),
                ])
                .split(screen[2]);
            f.render_widget(
                Paragraph::new("ESC  to quit")
                    .style(white)
                    .alignment(Alignment::Left),
                footer[0],
            );
            f.render_widget(
                Paragraph::new(face_frames[face_frame])
                    .style(bold_white)
                    .alignment(Alignment::Center),
                footer[1],
            );
        }).expect("Failed to draw terminal");
        if poll(Duration::from_millis(180)).expect("Failed to poll terminal events") {
            if let Event::Key(key_event) = read().expect("Failed to read terminal event") {
                match key_event.code {
                    KeyCode::Esc => {
                        let save_data = SaveState {
                            pdf_path: pdf_path.clone(),
                            current_batch: batch_start,
                            personal_best,
                            speed: latest_speed,
                            accuracy: latest_accuracy,
                        };
                        let serialized_data = bincode::serialize(&save_data).expect("Failed to serialize save data");
                        std::fs::File::create("save_file.bin")
                            .expect("Failed to create save file")
                            .write_all(&serialized_data)
                            .expect("Failed to write save data to file");
                        disable_raw_mode().expect("Failed to disable raw mode");
                        execute!(std::io::stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");
                        break;
                    }

                    // Capture the character from the user input in real time.
                    KeyCode::Char(c) => {
                        typed_chars.push(c);
                        face_frame = (face_frame + 1) % face_frames.len();
                    }
                    KeyCode::Enter => {
                        typed_chars.push('\n');
                        face_frame = (face_frame + 1) % face_frames.len();
                    }
                    _ => {}
                }
            }
        }
        if typed_chars.len() >= text.chars().count() {
            let final_speed = words_per_minute(&typed_chars, batch_start_time.elapsed());
            let final_accuracy = accuracy_percent(&typed_chars, &text);
            latest_speed = final_speed;
            latest_accuracy = final_accuracy;
            personal_best = personal_best.max(final_speed);
            typed_chars.clear();
            batch_start = batch_end;
            batch_start_time = Instant::now();
            ghost_batch_start = Instant::now();
        }
    }
}


    
fn main() {
    let saved_state = std::fs::read("save_file.bin")
        .ok()
        .and_then(|bytes| bincode::deserialize::<SaveState>(&bytes).ok());
    let saved_path = saved_state
        .as_ref()
        .filter(|state| validate_pdf_path(&state.pdf_path).is_ok())
        .map(|state| state.pdf_path.as_str());
    let Some(path_to_the_file_main) = select_pdf_path(saved_path) else {
        return;
    };

    let (current_batch, personal_best, speed, accuracy) = saved_state
        .filter(|state| state.pdf_path == path_to_the_file_main)
        .map(|state| {
            (
                state.current_batch,
                state.personal_best,
                state.speed,
                state.accuracy,
            )
        })
        .unwrap_or((0, 0, 0, 100));

    let sentences = match get_sentences(&path_to_the_file_main) {
        Ok(sentences) => sentences,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    start_TUI(
        sentences,
        path_to_the_file_main,
        current_batch,
        personal_best,
        speed,
        accuracy,
    );
}


// /home/dhruv/Downloads/test_for_app.pdf
