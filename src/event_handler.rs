use crate::cursor_position::CursorPosition;
use crate::words::Status::{Correct, Unmark, Wrong};
use crate::words::Word;

pub fn on_backspace(word: &mut Word, cursor: &mut CursorPosition) -> bool {
    for letter in word.letters.iter_mut().rev() {
        if letter.status == Correct || letter.status == Wrong {
            letter.status = Unmark;
            cursor.move_left();
            return true;
        }
    }
    false
}
pub fn on_keypress(
    word: &mut Word,
    c: char,
    did_mark_letter: &mut bool,
    cursor: &mut CursorPosition,
    correctly_pressed_letters: &mut usize,
    all_letter_pressed: &mut usize,
) -> bool {
    if *did_mark_letter {
        return true;
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
    cursor.move_right();

    if word
        .letters
        .iter()
        .all(|it| it.status == Correct || it.status == Wrong)
    {
        word.completed = true;
    };
    false
}
