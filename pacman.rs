use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize); // size of the grid
    let height = parse_input!(inputs[1], usize); // top left corner is (x=0, y=0)
    // TODO: Design gamestate struct
    // TODO: Create Pacman struct to nest in gamestate
    // TODO: Build Map from inputs here as
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string(); // one line of the grid: space " " is floor, pound "#" is wall
    }

    let mut game_state = State{..Default::default()}; // Create empty game_state

    for y in 0..height {
        game_state.board.push(Vec::new());
        for x in 0..width {
            game_state.board[y].push(-1 as i8);
        }
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_score = parse_input!(inputs[0], i32);
        let opponent_score = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pac_count = parse_input!(input_line, i32); // How many players you can see inclusive of self
        for i in 0..visible_pac_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let pac_id = parse_input!(inputs[0], i32); // pac number (unique within a team)
            let mine = parse_input!(inputs[1], i32); // true if this pac is yours
            let x = parse_input!(inputs[2], i32); // position in the grid
            let y = parse_input!(inputs[3], i32); // position in the grid
            //let type_id = inputs[4].trim().to_string(); // unused in wood leagues
            //let speed_turns_left = parse_input!(inputs[5], i32); // unused in wood leagues
            //let ability_cooldown = parse_input!(inputs[6], i32); // unused in wood leagues
        }


        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pellet_count = parse_input!(input_line, i32); // all pellets in sight
        for i in 0..visible_pellet_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            let value = parse_input!(inputs[2], i32); // amount of points this pellet is worth
            //eprintln!("({}, {}): {}", x, y, value);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("MOVE 0 15 10"); // MOVE <pacId> <x> <y>

        /*
        // Prints out current game_state.board
        for y in 0..height {
            for x in 0..width {
                eprint!("{} ", game_state.board[y][x]);
            }
            eprintln!("");
        }
        */

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
