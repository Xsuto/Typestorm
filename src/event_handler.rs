use crate::words::Status::{Correct, Unmark, Wrong};
use crate::words::Word;

pub fn on_backspace(word: &mut Word, pos: &mut i32) {
    let len = word.letters.len();
    let mut done = false;
    let mut i = 0;
    while i < len {
        if let Some(next) = word.letters.get(i + 1) {
            if (word.letters[i].status == Correct || word.letters[i].status == Wrong)
                && next.status == Unmark
            {
                word.letters[i].status = Unmark;
                *pos -= 1;
                done = true;
            }
        }
        i += 1;
    }
}
pub fn on_keypress(
    word: &mut Word,
    c: char,
    did_mark_letter: &mut bool,
    pos: &mut i32,
    correctly_pressed_letters: &mut i32,
    all_letter_pressed: &mut i32,
) -> bool {
    if *did_mark_letter {
        return true;
    }
    if word.completed {
        return false;
    }
    for letter in &mut word.letters {
        if letter.status == Unmark && letter.current_letter == c {
            letter.status = Correct;
            *did_mark_letter = true;
            *all_letter_pressed += 1;
            *correctly_pressed_letters += 1;
            break;
        }
        if letter.status == Unmark && letter.current_letter != c {
            letter.status = Wrong;
            *all_letter_pressed += 1;
            *did_mark_letter = true;
            break;
        }
    }
    *pos += 1;

    if word
        .letters
        .iter()
        .all(|it| it.status == Correct || it.status == Wrong)
    {
        word.completed = true;
    };
    return false;
}
