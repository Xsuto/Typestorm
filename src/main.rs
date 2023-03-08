use std::time::{Duration, Instant};

use clap::Parser;
use ncurses::*;

use crate::cursor_position::CursorPosition;
use crate::event_handler::{on_backspace, on_keypress};
use crate::words::show_words;
use crate::words::Status::Unmark;

mod cursor_position;
mod english1k_words;
mod english_words;
mod event_handler;
mod words;

enum ColorsPair {
    White = 1,
    Green = 2,
    Red = 3,
    RedSpace = 4,
}

fn init_ncurses() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    start_color();
    use_default_colors();
    init_pair(ColorsPair::White as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(ColorsPair::Green as i16, COLOR_GREEN, COLOR_BLACK);
    init_pair(ColorsPair::Red as i16, COLOR_RED, COLOR_BLACK);
    init_pair(ColorsPair::RedSpace as i16, COLOR_RED, COLOR_RED);
}

#[derive(clap::ValueEnum, Clone)]
pub enum WordsList {
    English,
    English1k,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 60)]
    timeframe: u64,

    #[arg(long = "max", default_value_t = 1000)]
    max_word_length: usize,

    #[arg(long = "min", default_value_t = 0)]
    min_word_length: usize,

    #[arg(short,long,value_enum, default_value_t = WordsList::English)]
    words_list: WordsList,
}

fn main() {
    let args = Args::parse();
    init_ncurses();
    let timeframe_in_secs = args.timeframe;
    let mut words =
        words::shuffle_and_get_words(&args.words_list, args.min_word_length, args.max_word_length);
    let mut cursor = CursorPosition::new();
    let mut now = Instant::now();
    let mut did_start_typing = false;
    let mut correctly_pressed_letters = 0;
    let mut all_letter_pressed = 0;

    show_words(&words, &mut cursor);
    while now.elapsed() < Duration::from_secs(timeframe_in_secs) || !did_start_typing {
        refresh();
        let c = getch();
        if c != ERR {
            clear();
            let c = c as u8 as char;

            // Start measuring time on first keypress
            if !did_start_typing {
                now = Instant::now();
                did_start_typing = true;
            }

            let mut did_mark_letter = false;
            for i in 0..words.len() {
                let word = &mut words[i];
                if word.completed {
                    continue;
                }
                // 127 is backspace
                if c as u8 == 127 {
                    // If on_backspace return false we have to modify word before him
                    if !on_backspace(word, &mut cursor) && i != 0 {
                        words[i - 1].letters.last_mut().unwrap().status = Unmark;
                        words[i - 1].completed = false;
                        if cursor.get_x() == 0 {
                            cursor.go_back_to_old_line();
                        } else {
                            cursor.move_left();
                        }
                    }
                    break;
                }
                // 9 == Tab Reset
                else if c as u8 == 9 {
                    words = words::shuffle_and_get_words(
                        &args.words_list,
                        args.min_word_length,
                        args.max_word_length,
                    );
                    did_start_typing = false;
                    now = Instant::now();
                    cursor = CursorPosition::new();
                    correctly_pressed_letters = 0;
                    all_letter_pressed = 0;
                    break;
                } else if on_keypress(
                    word,
                    c,
                    &mut did_mark_letter,
                    &mut cursor,
                    &mut correctly_pressed_letters,
                    &mut all_letter_pressed,
                ) {
                    break;
                }
            }
            show_words(&words, &mut cursor);
        }
    }
    // Cleanup ncurses
    endwin();

    let average_word_length = words
        .iter()
        .filter(|it| it.completed)
        .map(|it| it.letters.len())
        .sum::<usize>() as f64
        / words.iter().filter(|it| it.completed).count() as f64;
    println!(
        "Accuracy {}%",
        ((correctly_pressed_letters as f64 / all_letter_pressed as f64) * 100.0) as i64
    );
    println!(
        "WPM {}",
        ((all_letter_pressed as f64 / average_word_length) / (timeframe_in_secs as f64 / 60.0))
            as i64
    );
}
