use lopdf::Document;
use std::time::{Duration, Instant};
use std::path::Path;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{poll, read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::text::{Line, Span};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Clear, Paragraph, Wrap},
    Terminal,
};




// get the file path from the user
fn get_path_file() -> String{
    let mut path_to_the_file= String::new();
    let mut raw_path_input = String::new();

    while !Path::new(&raw_path_input).exists() {
        //clear the variable before the next run
        raw_path_input.clear();
        // Takes the user input and converts the string to the path
        if !Path::new(&raw_path_input).exists() {
            println!("Please enter the valid path to the pdf \n You want to practice typing(Make sure to use all the proper notations as per your OS)\n Enter the file path: ");
            std::io::stdin().read_line(&mut raw_path_input).expect("Failed to read line");
            raw_path_input = raw_path_input.trim().to_string();
        }
        //checking block for the file to be present / Valid or not 
        if Path::new(&raw_path_input).exists() && raw_path_input.ends_with(".pdf") {
            path_to_the_file = raw_path_input.clone();
            return path_to_the_file;
        } else {
            println!("The path you entered does not exist. Please try again.");
        }
    }
    return String::new();
}



//Open the file and get the lines from the pdf 
fn get_sentences(path_to_the_file:&str)-> Vec<String>{

    // extracting the text from the PDF
    let mut Doc_load = Document::load(path_to_the_file).expect("Failed to load PDF document");
    let mut extract_raw_page = Doc_load.extract_text(&[1]).expect("Failed to extract text from PDF document");
    let mut sentence: Vec<String> = Vec::new();
    
    // Loop to extract all non-empty sentences from the PDF.
    for line in extract_raw_page.lines() {
        let cleaned_line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if !cleaned_line.is_empty() {
            sentence.push(cleaned_line);
        }
    }
    return sentence
}   






//get the TUI running to get the user screen he wants 
fn start_TUI(sentences: Vec<String>){
    execute!(std::io::stdout(), EnterAlternateScreen).expect("Failed to enter alternate screen");
    enable_raw_mode().expect("Failed to enable raw mode");
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");
    let mut face_frame = 0usize;
    let mut typed_chars: Vec<char> = Vec::new();
    let mut total_typed_chars = 0usize;
    let mut batch_start = 0usize;
    let start_time = Instant::now();
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
        "(˶ᵔ ᵕ ᔁ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᔁ ᵕ ᔁ˶)",
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
        "(˶ᵔ ᵕ ᔁ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᔁ˶)",
        "(ꈍᴗꈍ)",
        "(ˆ‿ˆ)",
        "(•ω•)",
        "(•ᴗ•)",
        "(•‿•)",
        "(◕‿◕)",
        "(◠‿◠)",
        "(•̀ᴗ•́)",
        "(◕⩊◕)",
        "(˶ᵔ ᵕ ᔁ˶)",
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
            // Make the letters change color based on whether the character is correct or wrong.
            for target_line in text.split('\n') {
                let mut colored_captured_text = Vec::new();
                for target_char in target_line.chars() {
                    if let Some(typed_char) = typed_chars.get(character_index) {
                        if *typed_char == target_char {
                            colored_captured_text.push(Span::styled(
                                target_char.to_string(),
                                Style::default().fg(Color::Green),
                            ));
                        } else {
                            colored_captured_text.push(Span::styled(
                                target_char.to_string(),
                                Style::default().fg(Color::Red),
                            ));
                        }
                    } else {
                        colored_captured_text.push(Span::styled(
                            target_char.to_string(),
                            Style::default().fg(Color::White),
                        ));
                    }
                    character_index += 1;
                }
                colored_captured_lines.push(Line::from(colored_captured_text));
                character_index += 1;
            }

            let correct_count = typed_chars
                .iter()
                .zip(text.chars())
                .filter(|(typed_char, target_char)| **typed_char == *target_char)
                .count();
            let accuracy = if typed_chars.is_empty() {
                100
            } else {
                correct_count
                    .saturating_mul(100)
                    .checked_div(typed_chars.len())
                    .unwrap_or(0)
                    .clamp(0, 100)
            };
            let elapsed_minutes = start_time.elapsed().as_secs_f64() / 60.0;
            let speed = if total_typed_chars == 0 || elapsed_minutes == 0.0 {
                0
            } else {
                ((total_typed_chars as f64 / 5.0) / elapsed_minutes) as u64
            };
            
            f.render_widget(
                Paragraph::new(format!("WPM  {}   ACC  {}%", speed, accuracy))
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
                    disable_raw_mode().expect("Failed to disable raw mode");
                    execute!(std::io::stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");
                    break;
                    }

                    // Capture the character from the user input in real time.
                    KeyCode::Char(c) => {
                        typed_chars.push(c);
                        total_typed_chars += 1;
                        face_frame = (face_frame + 1) % face_frames.len();
                    }
                    KeyCode::Enter => {
                        typed_chars.push('\n');
                        total_typed_chars += 1;
                        face_frame = (face_frame + 1) % face_frames.len();
                    }
                    _ => {}
                }
            }
        }
        if typed_chars.len() >= text.chars().count() {
            typed_chars.clear();
            batch_start = batch_end;
        }
    }
}


    
fn main() {
    let path_to_the_file_main = get_path_file();
    let sentences = get_sentences(&path_to_the_file_main);
    start_TUI(sentences);
}


// /home/dhruv/Downloads/test_for_app.pdf
