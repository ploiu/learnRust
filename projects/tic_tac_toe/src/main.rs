use std::io;

fn main() {
    let mut spaces: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    // we'll use this to read which input a player wants
    let in_stream = io::stdin();
    let mut turn = -1;
    loop {
        clear_screen();
        get_input(&turn, &mut spaces, &in_stream);
        if check_win(&turn, &spaces) {
            print_board(&spaces);
            let winning_char = if turn == -1 { "x" } else { "o" };
            println!("Congratulations! {} won!", winning_char);
            break;
        } else if check_tie(&spaces) {
            print_board(&spaces);
            println!("BOOOOOOO! It's a tie...");
            break;
        }
        turn *= -1;
    }
    println!("Press any key to exit.");
    in_stream.read_line(&mut String::new()).unwrap();
}

fn clear_screen() {
    print!("\x1B[2J");
}

/*prints the board, using the passed vec to populate the spaces (15 x 7)
       |   |
       |   |
  -----+---+-----
       |   |
  -----+---+-----
       |   |
       |   |
*/
fn print_board(spaces: &[i32; 9]) {
    clear_screen();
    println!("{}", get_board(spaces, false));
}

/*
    returns the visual state of the board
*/
fn get_board(spaces: &[i32; 9], show_input_numbers: bool) -> String {
    // this spaces vec will always be 9 elements long
    let mut space_chars: Vec<String> = Vec::new();
    for index in 0..9 {
        let value = spaces[index];
        space_chars.push(if value == -1 {
            String::from("x")
        } else if value == 0 && !show_input_numbers {
            String::from(" ")
        } else if value == 0 && show_input_numbers {
            (index as i32).to_string()
        } else {
            String::from("o")
        })
    }
    let mut board = String::from("      |   |      \n");
    board += format!(
        "  {}   | {} |   {}   \n",
        space_chars[0], space_chars[1], space_chars[2]
    )
    .as_str();
    board += "-----+---+-----\n";
    board += format!(
        "  {}   | {} |   {}   \n",
        space_chars[3], space_chars[4], space_chars[5]
    )
    .as_str();
    board += "------+---+------\n";
    board += "      |   |      \n";
    board += format!(
        "  {}   | {} |   {}   \n",
        space_chars[6], space_chars[7], space_chars[8]
    )
    .as_str();
    board
}

/*
    reads the input for the passed turn into our spaces array based on the input
    @param turn the current turn (either -1 or 1)
    @param spaces the current state of the board
    @param in_stream Standard in, used to read input
*/
fn get_input(turn: &i32, spaces: &mut [i32; 9], in_stream: &io::Stdin) {
    let mut input = String::new();
    loop {
        println!("Input which space you want to occupy");
        // print the board with the num pad filling in the spaces to show the user which key does what, along with the actual board
        println!("{}", get_board(&spaces, true));
        in_stream.read_line(&mut input).unwrap();
        let number: i32 = match input.trim().parse::<u32>() {
            Ok(result) => {
                // make sure that we selected a valid number...there's probably a way better way of doing this instead of the 2 casts
                if result < 9 && spaces[result as usize] == 0 {
                    result as i32
                } else {
                    -1
                }
            }
            Err(_) => {
                println!("That is not a valid number.");
                -1
            }
        };
        // if the number is -1, that means we didn't input a valid number
        if number < 0 {
            continue;
        } else {
            // the user selected a valid number, so mark in the array and tell the loop we're all good
            spaces[number as usize] = *turn;
            break;
        }
    }
}

/**
 * Checks if the current turn has won the game
 */
fn check_win(turn: &i32, spaces: &[i32; 9]) -> bool {
    // once again, there's probably a better way than brute forcing it, but I don't know the math and this project isn't about knowing the best formulae - it's about writing stuff in rust
    let win_axes: [[i32; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 4, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [6, 4, 2],
    ];
    for axis in win_axes {
        let mut axes_match = true;
        for space in axis {
            axes_match = axes_match && spaces[space as usize] == *turn;
            // since we're in nested loops, the sooner we can skip an axis the better
            if !axes_match {
                break;
            }
        }
        // if axes_match is still true, that means that the row is lined up and we can just skip
        if axes_match {
            return axes_match;
        }
    }
    // none of the axes matched...
    false
}

/**
 * Checks if all the slots on the board are filled up.
 * Should be called only after check_win.
 */
fn check_tie(spaces: &[i32; 9]) -> bool {
    for index in 0..8 {
        if spaces[index] == 0 {
            return false;
        }
    }
    return true;
}
