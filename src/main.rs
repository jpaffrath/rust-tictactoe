use text_io::read;

mod board;

fn main() {
    let mut board: [char; 9] = ['1','2','3','4','5','6','7','8','9'];

    board::print(&board);

    let mut input: u8;
    let mut toggle = 1;

    while board::has_won(&mut board) == false {
        print!("User {}, please enter your field: ", toggle);
        input = read!();

        board::set_field(&mut board, input, &mut toggle);
        board::print(&board);
    }
}