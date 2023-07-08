pub fn print(board: &[char]) {
    println!();

    for i in 0..board.len() {
        match i {
            0 | 1 | 3 | 4 | 6 | 7 => print!("{} | ", board[i]),
            2 | 5 | 8 => println!("{}", board[i]),
            _ => ()
        }
    }

    println!();
}

pub fn set_field(board: &mut [char], field: u8, toggle: &mut u8) {
    board[usize::from(field-1)] = if toggle == &1 { 'X' } else { 'O' };
    *toggle = if toggle == &1 { 2 } else { 1 };
}

pub fn has_won(board: &mut [char]) -> bool {
    for i in 0..3 {
        let offset_column = i;
        if board[offset_column] == board[offset_column+3] &&
           board[offset_column] == board[offset_column+6] {
            return true;
        }

        let offset_row = i * 3;
        if board[offset_row] == board[offset_row+1] &&
           board[offset_row] == board[offset_row+2] {
            return true;
        }
    }

    if board[0] == board[4] && board[0] == board[8] ||
       board[2] == board[4] && board[2] == board[6] {
            return true;
    }
    
    false
}