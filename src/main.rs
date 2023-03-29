

struct Point {
    x: i32,
    y: i32,
}

//TODO: maybe put these in a HashSet and overload the equal operator,
// if the cars vector and speed are equal, don't store it
// TODO: would I care about the number of previous moves here? It seems that the lowest one would
//  be best b/c I have a time limit
struct Move {
    previous_commands: Vec<String>,
    speed: i32,
    cars_y: Vec<i32>,
    cars_x: i32
}



// it has to be calculate on every square the car could possibly have come from
fn main() {


    //TODO: what did I do?
    // I measured out the distance to each hole and calculated possible speeds for the bikes to get across.

    //IDEA: I can start at the back and work my way forwards
    // Highest speed possible is 'sqrt(start_speed^2 + 2 * track_len)' floor
    // Lowest speed possible is  'sqrt(start_speed^2 - 2 * track_len)' floor (if the inner part is negative then lowest is 1)
    // The first move requires to calculate every speed possible
    //  Need to try all possible speeds
    //  Also say I am working backwards as speed 5, I need to try pos (final-1),(final-2),(final-3),(final-4),(final-5)
    //  With a lane length up to 500 then, I could need to check 1-31 + 1-30 + 1-29 + 1-28 etc...; so its 4960 possibilities at 500 long
    // After the first move then I would need to check the minimum speed again (a lot would fall off every time), check for duplicates etc... as I work backwards


    //IDEA: Work from the front, find the pits and 'decide' what I can do to save the most cars, or at the very least save all the cars I can
    // Essentially I would need to branch every possible move every time
    // So 2 squares would be


    //So I only need to do it for one because the cars are a 'block'

    //n^2/2 + n/2 = x;
    //(sqrt(8x + 1) - 1)/2
    //500 ~ 31.12672920173694

    //Say constant speed of 1
    //Say holes can only be 1 wide
    //moves JUMP, UP, DOWN, WAIT
    //this is simpleish, its the same thing as the problem yesterday

    //Speed
    // Could jump too far and hit another hole
    // Could not jump far enough and not get over a hole
    // Could not go fast enough and not finish the track
    // Need to go as fast as possible while going over the holes
    // A speed of 20 on a track that is 20 long is impossible, it is a summation

}

//Only pass in valid moves
//TODO:
fn calculate_moves(
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32
) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();

    //TODO: so I think I should assume this move is valid and then ONLY push valid stuff to the board from it
    // so only push valid moves into the vector

    // Move {
    //     previous_commands: Vec<String>,
    //     speed: i32,
    //     cars_y: Vec<i32>,
    //     cars_x: i32
    //}

    //UP



    //TODO: if a lower speed is not possible, SPEED must be chosen
    //TODO: cannot go up if a car has x == 0
    //TODO: cannot go down if a car has x == 3
    //TODO: if this is a hole, then ?

    //SPEED
    //SLOW
    //JUMP
    //WAIT
    //UP
    //DOWN


    //TODO: do speeds

    moves
}

fn get_previous_valid_moves(
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();

    moves = push_up_if_valid(
        moves,
        lanes,
        current_move,
        starting_speed,
        min_cars_to_win,
    );

    moves = push_down_if_valid(
        moves,
        lanes,
        current_move,
        starting_speed,
        min_cars_to_win,
    );

    moves = push_wait_if_valid(
        moves,
        lanes,
        current_move,
        starting_speed,
        min_cars_to_win,
    );

    moves = push_jump_if_valid(
        moves,
        lanes,
        current_move,
        starting_speed,
        min_cars_to_win,
    );

    moves = push_slow_if_valid(
        moves,
        lanes,
        current_move,
        starting_speed,
        min_cars_to_win,
    );

    moves = push_speed_if_valid(
        moves,
        lanes,
        current_move,
        starting_speed,
        min_cars_to_win,
    );

    moves
}

fn push_up_if_valid(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {

    let prev_x = current_move.cars_x - current_move.speed;

    //If before start.
    if prev_x < 0 {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Cannot go up if a car is already at the top
        if y == 0 {
            return moves;
        }

        let prev_y = y - 1;
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_move.cars_x {
            if lanes[y as usize][i as usize] == b'0'
                || lanes[prev_y as usize][i as usize] == b'0'
            {
                continue 'y_check;
            }
        }

        cars_y_clone.push(prev_y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &current_move.speed
    );

    if !valid_move {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("UP".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: prev_x
        }
    );

    moves
}

fn push_down_if_valid(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {

    let prev_x = current_move.cars_x - current_move.speed;

    //If before start.
    if prev_x < 0 {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Cannot go up if a car is already at the top
        if y == 3 {
            return moves;
        }

        let prev_y = y + 1;
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_move.cars_x {
            if lanes[y as usize][i as usize] == b'0'
                || lanes[prev_y as usize][i as usize] == b'0'
            {
                continue 'y_check;
            }
        }

        cars_y_clone.push(prev_y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &current_move.speed
    );

    if !valid_move {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("DOWN".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: prev_x
        }
    );

    moves
}

fn push_wait_if_valid(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {

    let prev_x = current_move.cars_x - current_move.speed;

    //If before start.
    if prev_x < 0 {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_move.cars_x {
            if lanes[y as usize][i as usize] == b'0'
            {
                continue 'y_check;
            }
        }

        cars_y_clone.push(y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &current_move.speed
    );

    if !valid_move {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("WAIT".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: prev_x
        }
    );

    moves
}

fn push_jump_if_valid(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {

    let prev_x = current_move.cars_x - current_move.speed;

    //If before start.
    if prev_x < 0 {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    for &y in &current_move.cars_y {
        //Only the hole at the start of the jump needs to be taken into account.
        if lanes[y as usize][prev_x as usize] == b'0' {
            continue;
        }

        cars_y_clone.push(y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &current_move.speed
    );

    if !valid_move {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("JUMP".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: prev_x
        }
    );

    moves
}

fn push_slow_if_valid(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {

    //If SLOW was called, it WAS going at a faster speed. So to reverse the move, add 1.
    let new_speed = current_move.speed + 1;
    let prev_x = current_move.cars_x - new_speed;

    //If before start.
    if prev_x < 0 {
        return moves;
    }

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &new_speed
    );

    if !valid_move {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_move.cars_x {
            if lanes[y as usize][i as usize] == b'0'
            {
                continue 'y_check;
            }
        }

        cars_y_clone.push(y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("SLOW".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: new_speed,
            cars_y: cars_y_clone,
            cars_x: prev_x
        }
    );

    moves
}

fn push_speed_if_valid(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {

    //If SPEED was called, it WAS going at a slow speed. So to reverse the move, subtract 1.
    let new_speed = current_move.speed - 1;
    let prev_x = current_move.cars_x - new_speed;

    //If before start.
    if prev_x < 0 {
        return moves;
    }

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &new_speed
    );

    if !valid_move {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_move.cars_x {
            if lanes[y as usize][i as usize] == b'0'
            {
                continue 'y_check;
            }
        }

        cars_y_clone.push(y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("SPEED".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: new_speed,
            cars_y: cars_y_clone,
            cars_x: prev_x
        }
    );

    moves
}


fn speed_is_valid(
    x_pos: &i32,
    starting_speed: &i32,
    current_speed: &i32
) -> bool {
    let x_pos_f = *x_pos as f32;
    let starting_speed_f = *starting_speed as f32;
    //sqrt(start_speed^2 + 2 * track_len)
    let highest_possible_speed = ((starting_speed_f * starting_speed_f) + 2.0 * (x_pos_f+1.0)).sqrt() as i32;

    //sqrt(start_speed^2 - 2 * track_len)
    let lowest_inner_part = (starting_speed_f * starting_speed_f) - 2.0 * (x_pos_f+1.0);
    let lowest_possible_speed = if lowest_inner_part <= 1.0 {
        lowest_inner_part.sqrt() as i32
    } else {
        1
    };

    if *current_speed < lowest_possible_speed || highest_possible_speed < *current_speed {
        false
    } else {
        true
    }
}

// fn move_is_valid(
//     lanes: &Vec<Vec<u8>>,
//     current_move: &Move,
//     starting_speed: i32,
//     min_cars_to_win: i32,
// )-> bool {
//
//
// }


//Achievement (#8) reverse command orders
// let mut instructions = Vec::<String>::from([
//     "WAIT".to_string(),
//     "JUMP".to_string(),
//     "SPEED".to_string(),
//     "JUMP".to_string(),
//     "WAIT".to_string(),
//     "WAIT".to_string(),
// ]);