use std::io;
use std::collections::VecDeque;

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
     * Builds only the original walls with no pacman or pellet values. Empty game board. POSSIBILITY: Remove and read into
     * into unused variable and then generate board more efficiently if turn one is too slow.
    */
    for y in 0..height as usize { // From 0 to our known height
        let mut input_line = String::new(); // Create a new string
        io::stdin().read_line(&mut input_line).unwrap(); // Save the read line to input_line as String
        // one line of the grid: space " " is floor, pound "#" is wall
        let row_chars: Vec<_> = input_line.trim_matches('\n').to_string().chars().collect();  // Cleans up input line, makes it a String, makes it an array, converts it to a char iterator
        game_state.board.push(Vec::new()); // Pushes a brand new empty row to our game board, as we are storing a vector of rows, creating the 2D array equivalent
        for ch in row_chars { // For each char that we collected from the current input_line
            if ch == '#' { // If it is a '#', it is a wall, which we store as -1
                //game_state.board[y].push(-1);
                game_state.board[y].push(Tile{value: -1.0,..Default::default()});
            } else { // Otherwise, it is an empty space that will later be filled by some pellet value
                //game_state.board[y].push(0);
                game_state.board[y].push(Tile{value: 0.0,..Default::default()});

            }
        }
    }

    populate_neighbors(&mut game_state);

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
         * TODO: Change how player/enemy pacs updated, don't remake vector'''''''s each time
        */
        for i in 0..visible_pac_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let pac_id = parse_input!(inputs[0], usize); // pac number (unique within a team)
            let mine = parse_input!(inputs[1], usize); // true if this pac is yours
            let x = parse_input!(inputs[2], usize); // position in the grid
            let y = parse_input!(inputs[3], usize); // position in the grid
            game_state.board[y][x].value = 0.0; // Pacman ate pellet, remove from board

            match mine { // Is the current pac mine?
                1 => game_state.player_pacs.push(Pacman{id: pac_id, x: x, y: y, dest_x:10, dest_y:15}), // Yes, add to player_pacs
                _ => game_state.enemy_pacs.push(Pacman{id: pac_id, x: x, y: y, dest_x:0, dest_y:0}) // Nope, add it to enemy pacs
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
            let value = parse_input!(inputs[2], f32); // amount of points this pellet is worth
            game_state.board[y][x].value = value; // Assign value to given coordinate on board

        }

        /*
         * Updates desintations for each pacman in
         * game_state.player_pacs vector
         * and builds output String object
        */
        let mut print_me = String::new(); // String object to build
        for i in 0..game_state.player_pacs.len() as usize { // For all pacman in player_pacs
            breadth_first_search(&mut game_state, i);

            decide_destination(&mut game_state, i); // Update the pacman in game_state player pacman vector
            let pac = game_state.player_pacs[i]; // Current pac we're looking at
            let pac_move_info = format!("MOVE {} {} {}", pac.id, pac.dest_x, pac.dest_y); // Builds the pacman print statement
            print_me.push_str(&pac_move_info); // Adds the current pacman move information to print_me
            if i != game_state.player_pacs.len() - 1 { // If not the last pacman index
                print_me.push_str("|") // Add a pipe to seperate inputs
            }
        }
        eprintln!("{} {}", game_state.board[5][5].player_pacs[0], game_state.board[5][5].player_pacs[1]);
        println!("{}", print_me); // Prints built String object

        print_board(&game_state.board, width, height); // Prints current board from game_state
    }
}

/*
 * print_board() - Print referenced board pretty
 * Pass in a given board of Vec<Vec<i8>>, and the width + height of board
 * Each coordinate location has two character spots to be taken up
 * Possible values are 0, 1, -1, and 10, which are automatically spaced properly
 * to make an easy to read output for the console each turn
*/
fn print_board(board: &Vec<Vec<Tile>>, width: usize, height: usize) {
    for y in 0..height as usize {
        for x in 0..width as usize {
            let tile = board[y][x].value;
            if tile == 1.0 { // It's a pellet! print '.'
                eprint!(". ");
            } else if tile == -1.0 { // Wall (-1), so print nothing
                eprint!("  ", );
            } else if tile == 0.0 { // Eaten pellet, print  '_'
                eprint!("_ ");
            } else { // Value is 10
                eprint!("$ "); // Big money big money
            }
        }
        eprintln!(""); // Ends the printing of the current row
    }
}

/*
 * decide_desintation() updates the pacman indexed at state.player_pacs[index]
 * goal destination values of the current state
 * Finds the nearest big pip if it exists, otherwise find closest pip
*/
fn decide_destination(state: &mut State, index: usize) {
    let pac_id = state.player_pacs[index].id; // Get pac_id of currently indexed pac
    for i in 0..state.player_pacs.len() as usize {
        if state.player_pacs[i].id == pac_id {
            /**
             *  if matching id want to iterate over board to find matching circumstance
             *  1. Closest big pellet
             *  2. Closest pellet
             *  3. random location
             */
            let curr_x = state.player_pacs[i].x;
            let curr_y = state.player_pacs[i].y;
            let mut big_score = 0.0;
            let mut closest_dist = 100000;
            let mut goal_x = 0;
            let mut goal_y = 0;
            for y in 0..state.board.len() { // Iterate over board to find either closest pellet or big pellet
                for x in 0..state.board[y].len() {
                    if state.board[y][x].value != -1.0 { // If not wall
                        if state.board[y][x].value > big_score {
                            big_score = state.board[y][x].value;
                            goal_x = x;
                            goal_y = y;
                            closest_dist = calculate_distance(curr_y, curr_x, goal_y, goal_x); // Manhattan distance from new goal
                        } else if state.board[y][x].value == big_score {
                            let local_dist = calculate_distance(curr_y, curr_x, y, x); // Manhattan distance from coordinate
                            if local_dist < closest_dist { // If the distance to current coordinate is closer than what we have, update
                                goal_x = x;
                                goal_y = y;
                                closest_dist = local_dist;
                            }
                        }
                    }
                }
            }
            //eprintln!("{} {} {}", goal_x, goal_y, big_score);
            state.player_pacs[i].dest_x = goal_x;
            state.player_pacs[i].dest_y = goal_y;
        }
    }
}

/*
 * Simple manhattan distance calculator
 * Processes all coordinates as usize
 * Checks for and eliminates the possibility of negative operations
*/
fn calculate_distance(curr_y: usize, curr_x: usize, goal_y: usize, goal_x: usize) -> usize {
    let mut diff_y: usize = 0; // Preemptively declare as usize
    let mut diff_x: usize = 0; // Preemptively declare as usize

    // Calculate diff_y
    if curr_y > goal_y {
        diff_y = curr_y - goal_y;
    } else {
        diff_y = goal_y - curr_y;
    }

    // Calculate diff_x
    if curr_x > goal_x {
        diff_x = curr_x - goal_x;
    } else {
        diff_x = goal_x - curr_x;
    }

    return diff_x + diff_y; // Manhattan distance
}

/*
 * Fix references for comparison on 229 and 241

fn breadth_first_search(state: &mut State, index: usize){
    let mut to_explore = VecDeque::new();
    let mut curr_x = state.player_pacs[index].x;
    let mut curr_y = state.player_pacs[index].y;
    let mut curr_tile = &mut state.board[curr_y][curr_x];
    curr_tile.player_pacs[index] = 1;
    to_explore.push_back(curr_tile); // Push current tile
    //to_explore.push_back(&mut state.board[curr_tile.neighbors[tile].1][curr_tile.neighbors[tile].0]);
    /*
    for tile in 0..curr_tile.neighbors.len() {
        if state.board[curr_tile.neighbors[tile].1][curr_tile.neighbors[tile].0].player_pacs[index] == 0 { // Has the player visited this spot?
            to_explore.push_back(&mut state.board[curr_tile.neighbors[tile].1][curr_tile.neighbors[tile].0]);
        }
    }
    */
    let mut distance = 1;
    while !to_explore.is_empty() { // This is probably really inefficent, but was easiest way i could think of to maintain distance count
        let count = to_explore.len();
        for i in 0..count {
            curr_tile = to_explore[i];
            curr_tile.player_pacs[index] = distance;
            for tile in 0..curr_tile.neighbors.len() {
                let val = 0;
                if state.board[curr_tile.neighbors[tile].1][curr_tile.neighbors[tile].0].player_pacs[index] == val {
                    to_explore.push_back(&mut state.board[curr_tile.neighbors[tile].1][curr_tile.neighbors[tile].0]);
                }
            }
        }
        for i in 0..count{
            to_explore.remove(i);
        }
        distance = distance + 1;
    }
}
*/

/**
 * Populates neighbor vec in each tile
 * TODO: Handle board wrap around's
 */
fn populate_neighbors(state: &mut State){
    let mut bound_up = false;
    let mut bound_down = false;
    let mut bound_left = false;
    let mut bound_right = false;
    for y in 0..state.board.len(){
        for x in 0..state.board[y].len(){
            if state.board[y][x].value == 0.0{
                if y == 0 || state.board[y-1][x].value < 0.0 {
                    bound_up = true;
                }
                if y == state.board.len()-1 || state.board[y+1][x].value < 0.0 {
                    bound_down =true;
                }
                if x == 0 || state.board[y][x-1].value < 0.0 {
                    bound_left = true;
                }
                if x == state.board[y].len()-1 || state.board[y][x+1].value < 0.0 {
                    bound_right = true;
                }
                if !bound_up {
                    state.board[y][x].neighbors.push((x,y-1));
                }
                if !bound_down {
                    state.board[y][x].neighbors.push((x,y+1));
                }
                if !bound_left {
                    state.board[y][x].neighbors.push((x-1,y));
                }
                if !bound_right {
                    state.board[y][x].neighbors.push((x+1,y));
                }
            }
        }
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
    player_pacs: Vec<Pacman>, // TODO: Change to array of 5 like in struct Tile. Would have to change the vec.push() earlier in code
    enemy_pacs: Vec<Pacman>,
    player_score: usize,
    enemy_score: usize,
    board: Vec<Vec<Tile>>
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
#[derive(Copy, Clone)]
struct Pacman {
    x: usize,
    y: usize,
    id: usize,
    dest_x: usize,
    dest_y: usize
}

/*
 * Default constructor for Pacman struct
 * x = 0; y = 0
 * id = 1000
*/
impl Default for Pacman {
    fn default() -> Pacman {
        return Pacman{x: 0, y: 0, id: 1000, dest_x: 0, dest_y: 0}
    }
}


struct Tile {
	neighbors: Vec<(usize, usize)>,
	player_pacs: [usize; 5],
	enemy_pacs: [usize; 5],
    //origin_neighbor: Tile,
	value: f32 //Negative is a wall, 0 for taken, 1 for pellet, 10 for big
}

impl Default for Tile {
	fn default() -> Tile {
		return Tile{neighbors: Vec::new(), player_pacs: [0 as usize; 5], enemy_pacs: [0 as usize; 5], value: -1.0}
	}
}

// Lines taken from pacman reading loop, not used in wood leagues will add in later.
//let type_id = inputs[4].trim().to_string(); // unused in wood leagues
//let speed_turns_left = parse_input!(inputs[5], i32); // unused in wood leagues
//let ability_cooldown = parse_input!(inputs[6], i32); // unused in wood leagues


// decision making pseudo code

// if no_enemies
//     if big_pellet
//         move to closest one
//     else
//         calculate_paths of distance d
//         move best path
// else
//     calculate_paths with probabilities of distance d
//     if enemy within range r && cooldown == 0 && not in killing form
//         switch to killing form
//     else if enemy within range r && in killing form
//         chase enemy
//     else if enemy outside of range big_r && cooldown == 0 this one is optional
//         speed boost
//     else
//         pick best path

/*
 * Path evaluation equation
 *
 * if enemy
 *    val of tile = pellet_score - prob_enemy_taking
 * else
 *    val of tile = pellet_score
 *
 * val of path = sum of tiles along path
*/
