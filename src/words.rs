use lazy_static::lazy_static;
use ncurses::{addstr, attron, COLOR_PAIR, refresh};
use rand::seq::SliceRandom;

use crate::{ColorsPair, WordsList};
use crate::cursor_position::CursorPosition;

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

impl Word {
    pub fn size(&self) -> usize {
        self.letters.iter().len()
    }
}

#[derive(Debug)]
pub struct Word {
    pub letters: Vec<Letter>,
    pub completed: bool,
}

#[derive(Debug)]
pub struct Words {
    pub data: Vec<Word>,
    start: usize,
    end: usize,
    previous_start: Vec<usize>,
    previous_end: Vec<usize>,
    line_position: usize,
    margin: usize,
}

impl Words {
    pub fn new(data: Vec<Word>, terminal_size: usize, margin: usize) -> Self {
        let mut end = 0;
        let mut it = 0;
        for word in data.iter() {
            if it + word.size() + 2 * margin < terminal_size {
                it += word.size();
                end += 1;
            } else {
                break;
            }
        }
        Self {
            data,
            start: 0,
            end,
            previous_start: vec![],
            previous_end: vec![],
            line_position: 0,
            margin,
        }
    }
    pub fn get_words_to_display(
        &mut self,
        cursor: &mut CursorPosition,
        terminal_width: usize,
    ) -> &[Word] {
        if self.start != 0 && self.data.iter().filter(|it| it.completed).count() < self.start {
            cursor.go_back_to_old_line();
            self.line_position -= 1;
            self.start = self.previous_start[self.line_position];
            self.end = self.previous_end[self.line_position];
        }
        if self.data.iter().filter(|it| it.completed).count() > self.end - 1 {
            let mut it = 0;
            self.previous_start.push(self.start);
            self.previous_end.push(self.end);
            self.line_position += 1;

            self.start = self.end;
            cursor.move_to_new_line();
            for word in self.data.iter().skip(self.end) {
                if it + word.size() + 2 * self.margin < terminal_width {
                    it += word.size();
                    self.end += 1;
                } else {
                    break;
                }
            }
        }
        &self.data[self.start..self.end]
    }
    pub fn show_words(&mut self, cursor: &mut CursorPosition, terminal_width: usize) {
        show_margin(self.margin);

        for word in self.get_words_to_display(cursor, terminal_width) {
            for letter in &word.letters {
                match &letter.status {
                    Status::Unmark => {
                        addstr(&letter.current_letter.to_string()[..]);
                    }
                    Status::Correct => {
                        show_correct_letter(letter.current_letter);
                    }
                    Status::Wrong => {
                        show_wrong_letter(letter.current_letter);
                    }
                }
            }
        }

        show_margin(self.margin);
        cursor.display();
        refresh();
    }
}

pub fn shuffle_and_get_words(
    words_list: &WordsList,
    min_word_length: usize,
    max_word_length: usize,
    terminal_width: usize,
    margin: usize
) -> Words {
    let mut words = match words_list {
        WordsList::English => Vec::from(crate::english_words::WORDS),
        WordsList::English1k => Vec::from(crate::english1k_words::WORDS),
    };
    words.shuffle(&mut rand::thread_rng());
    let data = words
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
        .collect::<Vec<Word>>();
    Words::new(data, terminal_width,margin)
}

fn show_correct_letter(word: char) {
    attron(COLOR_PAIR(ColorsPair::Green as i16));
    addstr(&word.to_string()[..]);
    attron(COLOR_PAIR(ColorsPair::White as i16));
}

fn show_wrong_letter(word: char) {
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

lazy_static! {
    static ref MARGIN: String = String::from(" ").repeat(1024);
}

fn show_margin(margin: usize) {
    // NOTE: Program will crash if margin is bigger then 1024 but I is not a realistic scenario.
    addstr(&MARGIN[0..margin]);
}
