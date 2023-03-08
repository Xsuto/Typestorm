use ncurses::{addstr, attron, refresh, COLOR_PAIR};
use rand::seq::SliceRandom;

use crate::cursor_position::CursorPosition;
use crate::{ColorsPair, WordsList};

#[derive(PartialEq, Debug)]
pub enum Status {
    Unmark,
    Correct,
    Wrong,
}
#[derive(Debug)]
pub struct Letter {
    pub current_letter: char,
    pub status: Status,
}

#[derive(Debug)]
pub struct Word {
    pub letters: Vec<Letter>,
    pub completed: bool,
}

pub fn shuffle_and_get_words(
    words_list: &WordsList,
    min_word_length: usize,
    max_word_length: usize,
) -> Vec<Word> {
    let mut words = match words_list {
        WordsList::English => Vec::from(crate::english_words::WORDS),
        WordsList::English1k => Vec::from(crate::english1k_words::WORDS),
    };
    words.shuffle(&mut rand::thread_rng());
    words
        .iter()
        .filter(|it| it.len() < max_word_length && it.len() >= min_word_length)
        .map(|it| {
            let mut letters = it
                .chars()
                .map(|letter| Letter {
                    current_letter: letter,
                    status: Status::Unmark,
                })
                .collect::<Vec<Letter>>();
            letters.push(Letter {
                current_letter: ' ',
                status: Status::Unmark,
            });
            Word {
                completed: false,
                letters,
            }
        })
        .collect::<Vec<Word>>()
}

fn word_in_green(word: char) {
    attron(COLOR_PAIR(ColorsPair::Green as i16));
    addstr(&word.to_string()[..]);
    attron(COLOR_PAIR(ColorsPair::White as i16));
}

fn word_in_red(word: char) {
    if word == ' ' {
        attron(COLOR_PAIR(ColorsPair::RedSpace as i16));
        addstr(" ");
        attron(COLOR_PAIR(ColorsPair::White as i16));
        return;
    }
    attron(COLOR_PAIR(ColorsPair::Red as i16));
    addstr(&word.to_string()[..]);
    attron(COLOR_PAIR(ColorsPair::White as i16));
}

pub fn show_words(words: &[Word], pos: &mut CursorPosition) {
    let it = words.iter().filter(|it| it.completed).count() / 10;

    if it != pos.get_line_position() {
        pos.move_to_new_line();
    }

    for word in words
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx >= it * 10 && *idx < (it + 1) * 10)
        .map(|(_, word)| word)
    {
        for letter in &word.letters {
            match &letter.status {
                Status::Unmark => {
                    addstr(&letter.current_letter.to_string()[..]);
                }
                Status::Correct => {
                    word_in_green(letter.current_letter);
                }
                Status::Wrong => {
                    word_in_red(letter.current_letter);
                }
            }
        }
    }
    pos.display();
    refresh();
}
