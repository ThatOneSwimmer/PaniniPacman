use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    // Read in static information about the board
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize); // size of the grid
    let height = parse_input!(inputs[1], usize); // top left corner is (x=0, y=0)

    let mut game_state = State{..Default::default()}; // Will be updating this each turn with current known information and values

    /*
     * Generates board in a likely overcomplicated way, but reads in lines that the game wants you to.
     * Builds only the original walls with no pacman or pellet values. Empty game board
    */
    for y in 0..height as usize { // From 0 to our known height
        let mut input_line = String::new(); // Create a new string
        io::stdin().read_line(&mut input_line).unwrap(); // Save the read line to input_line as String
        // one line of the grid: space " " is floor, pound "#" is wall
        let row_chars: Vec<_> = input_line.trim_matches('\n').to_string().chars().collect();  // Cleans up input line, makes it a String, makes it an array, converts it to a char iterator
        game_state.board.push(Vec::new()); // Pushes a brand new empty row to our game board, as we are storing a vector of rows, creating the 2D array equivalent
        for ch in row_chars { // For each char that we collected from the current input_line
            if ch == '#' { // If it is a '#', it is a wall, which we store as -1
                game_state.board[y].push(-1);
            } else { // Otherwise, it is an empty space that will later be filled by some pellet value
                game_state.board[y].push(0);
            }
        }
    }

    // Main game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_score = parse_input!(inputs[0], i32);
        let opponent_score = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pac_count = parse_input!(input_line, i32); // How many players you can see inclusive of self


        game_state.player_pacs = Vec::new(); // Clear player_pacs
        game_state.enemy_pacs = Vec::new(); // Clear enemy_pacs

        /*
         * Reads in information for all pacman on the board
         * Saves all player and enemy information to player_pacs and enemy_pacs respectively
        */
        for i in 0..visible_pac_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let pac_id = parse_input!(inputs[0], usize); // pac number (unique within a team)
            let mine = parse_input!(inputs[1], usize); // true if this pac is yours
            let x = parse_input!(inputs[2], usize); // position in the grid
            let y = parse_input!(inputs[3], usize); // position in the grid
            game_state.board[y][x] = 0; // Pacman ate pellet, remove from board

            match mine { // Is the current pac mine?
                1 => game_state.player_pacs.push(Pacman{id: pac_id, x: x, y: y}), // Yes, add to player_pacs
                _ => game_state.enemy_pacs.push(Pacman{id: pac_id, x: x, y: y}) // Nope, add it to enemy pacs
            }
        }

        /*
         * Reads in all pellets on map and updates board with values
        */
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pellet_count = parse_input!(input_line, i32); // all pellets in sight
        for i in 0..visible_pellet_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], usize); //redefined as usize
            let y = parse_input!(inputs[1], usize);
            let value = parse_input!(inputs[2], i8); // amount of points this pellet is worth

            game_state.board[y][x] = value; // Assign value to given coordinate on board
        }

        // Printing only one pacman for each team
        eprintln!("Player Pac: ({}, {})   Enemy Pac: ({}, {})",
        game_state.player_pacs[0].x, game_state.player_pacs[0].y,
        game_state.enemy_pacs[0].x, game_state.enemy_pacs[0].y);

        print_board(&game_state.board, width, height); // Prints current board from game_state

        println!("MOVE 0 15 10"); // MOVE <pacId> <x> <y>
    }
}

/*
 * print_board() - Print referenced board pretty
 * Pass in a given board of Vec<Vec<i8>>, and the width + height of board
 * Each coordinate location has two character spots to be taken up
 * Possible values are 0, 1, -1, and 10, which are automatically spaced properly
 * to make an easy to read output for the console each turn
*/
fn print_board(board: &Vec<Vec<i8>>, width: usize, height: usize) {
    for y in 0..height as usize {
        for x in 0..width as usize {
            let tile = board[y][x];
            if tile == 1 { // It's a pellet! print '.'
                eprint!(". ");
            } else if tile == -1 { // Wall (-1), so print nothing
                eprint!("  ", );
            } else if tile == 0 { // Eaten pellet, print  '_'
                eprint!("_ ");
            } else { // Value is 10
                eprint!("$ "); // Big money big money
            }
        }
        eprintln!(""); // Ends the printing of the current row
    }
}

/*
 * struct State used to keep track of game
 *
 * board Vec<Vec<i8>> values:
 * Wall = -1 TODO: check if -1 breaks
 * No pip = 0
 * Pip = 1
 * BIGPIP = 10
*/
struct State {
    player_pacs: Vec<Pacman>,
    enemy_pacs: Vec<Pacman>,
    player_score: usize,
    enemy_score: usize,
    board: Vec<Vec<i8>>
}

/*
 * Default constructor for State struct
*/
impl Default for State {
    fn default() -> State {
        return State{ player_pacs: Vec::new(), enemy_pacs: Vec::new(), player_score: 0, enemy_score: 0, board: Vec::new() }
    }
}

/*
 * Used to store individual Pacman information
 * x, y, and id
*/
struct Pacman {
    x: usize,
    y: usize,
    id: usize,
}

/*
 * Default constructor for Pacman struct
 * x = 0; y = 0
 * id = 1000
*/
impl Default for Pacman {
    fn default() -> Pacman {
        return Pacman{x: 0, y: 0, id: 1000}
    }
}

// Lines taken from pacman reading loop, not used in wood leagues will add in later.
//let type_id = inputs[4].trim().to_string(); // unused in wood leagues
//let speed_turns_left = parse_input!(inputs[5], i32); // unused in wood leagues
//let ability_cooldown = parse_input!(inputs[6], i32); // unused in wood leagues
