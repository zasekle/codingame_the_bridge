use std::collections::HashSet;
use std::io;
use std::hash::{Hash, Hasher};
use std::cmp;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum MoveValues {
    SPEED,
    JUMP,
    UP,
    DOWN,
    SLOW,
    WAIT,
}

#[derive(Debug, Clone)]
struct Move {
    previous_commands: Vec<MoveValues>,
    speed: i32,
    cars_y: Vec<i32>,
    cars_x: i32,
    previous_moves: Vec<Move>,
}

impl Hash for Move {
    fn hash<H>(&self, h: &mut H)
        where
            H: Hasher,
    {
        self.cars_x.hash(h)
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        if self.cars_y.len() != other.cars_y.len() {
            return false;
        }

        for i in 0..self.cars_y.len() {
            if self.cars_y[i] != other.cars_y[i] {
                return false;
            }
        }

        self.cars_x == other.cars_x
            && self.speed == other.speed
    }
}

impl Eq for Move {}

// it has to be calculate on every square the car could possibly have come from
fn main() {

    // 1 2 3 3 4 5 6 7
    //                     10                  20                  30
    // 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9
    //                                                           |

    // speeds: 1 2 3 4
    //      x: 1 3 6 10

    // speeds: 1 2 3 3
    //      x: 1 3 6 9
    //NOTE: x is not actually the vector index, it is the space between the vector elements. So
    // the same concept as an iterator.
    // #1
    let lanes: Vec<Vec<u8>> = Vec::from([
        "..............................".as_bytes().to_vec(),
        "..............................".as_bytes().to_vec(),
        "...........0..................".as_bytes().to_vec(),
        "..............................".as_bytes().to_vec(),
    ]);
    let starting_speed = 0;
    let min_cars_to_win = 1;
    let cars_starting_y = Vec::<i32>::from([2]);

    //#2
    // let lanes: Vec<Vec<u8>> = Vec::from([
    //     "..........000......0000..............000000.............".as_bytes().to_vec(),
    //     "..........000......0000..............000000.............".as_bytes().to_vec(),
    //     "..........000......0000..............000000.............".as_bytes().to_vec(),
    //     "..........000......0000..............000000.............".as_bytes().to_vec(),
    // ]);
    // let starting_speed = 1;
    // let min_cars_to_win = 4;
    // let cars_starting_y = Vec::<i32>::from([0,1,2,3]);

    //#8
    // let lanes: Vec<Vec<u8>> = Vec::from([
    //     ".............0.............0........".as_bytes().to_vec(),
    //     "..............0.............0.......".as_bytes().to_vec(),
    //     "...............0.............0......".as_bytes().to_vec(),
    //     "................0..........0000.....".as_bytes().to_vec(),
    // ]);
    // let starting_speed = 6;
    // let min_cars_to_win = 3;
    // let cars_starting_y = Vec::<i32>::from([0, 1, 2, 3]);

    //#11
    // let lanes: Vec<Vec<u8>> = Vec::from([
    //     "...0........0........0000.....".as_bytes().to_vec(),
    //     "....00......0.0...............".as_bytes().to_vec(),
    //     ".....000.......00.............".as_bytes().to_vec(),
    //     ".............0.0..............".as_bytes().to_vec(),
    // ]);
    // let starting_speed = 3;
    // let min_cars_to_win = 1;
    // let cars_starting_y = Vec::<i32>::from([0, 1, 2, 3]);

    //#12
    // let lanes: Vec<Vec<u8>> = Vec::from([
    //     "................000000000........00000........000.............00.".as_bytes().to_vec(),
    //     ".0.0..................000....000......0.0..................00000.".as_bytes().to_vec(),
    //     "....000.........0.0...000................000............000000.0.".as_bytes().to_vec(),
    //     "............0.000000...........0000...............0.0.....000000.".as_bytes().to_vec(),
    // ]);
    // let starting_speed = 1;
    // let min_cars_to_win = 1;
    // let cars_starting_y = Vec::<i32>::from([1, 2]);

    //TODO: why is this so much faster?
    // I want to count the leaves in both approaches
    // #8 should be good for it because the breadth first will calculate (all?) of them
    //TODO: maybe return all possible successful operations and see what happens? how many there are and how long it takes?

    let start = Instant::now();

    // let completed_moves = breadth_first_search(
    //     &lanes,
    //     &min_cars_to_win,
    //     &starting_speed,
    //     &cars_starting_y,
    // );


    let max_search_time_ms = 140;
    //TODO: this will find ALL possible paths, probably set it to find only one
    // except I want all of them occasionally, maybe check the runtime and cancel everything if <5ms before the 150ms deadline is reached
    let completed_moves = depth_first_search(
        &lanes,
        &min_cars_to_win,
        &starting_speed,
        &cars_starting_y,
        &max_search_time_ms
    );

    //TODO: sort completed moves, return one with most cars remaining

    let mut completed_moves_vec: Vec<Move> = completed_moves.into_iter().collect();
    completed_moves_vec.sort_by(|a, b| b.cars_y.len().partial_cmp(&a.cars_y.len()).unwrap());
    let mut instructions = Vec::<String>::new();

    for command in &completed_moves_vec[0].previous_commands {
        let final_cmd = match *command {
            MoveValues::SPEED => {"SPEED"}
            MoveValues::JUMP => {"JUMP"}
            MoveValues::UP => {"UP"}
            MoveValues::DOWN => {"DOWN"}
            MoveValues::SLOW => {"SLOW"}
            MoveValues::WAIT => {"WAIT"}
        };

        instructions.push(final_cmd.to_string());
    }

    println!("{:?}", instructions);

    let duration = start.elapsed();

    println!("Run time: {:?}", duration);
    // println!("completed_moves.len() {}", completed_moves.len());
    //
    // for single_move in completed_moves {
    //     println!("{:?}", single_move);
    // }
}

fn depth_first_search(
    lanes: &Vec<Vec<u8>>,
    min_cars_to_win: &i32,
    starting_speed: &i32,
    cars_starting_y: &Vec<i32>,
    max_search_time_ms: &u128
) -> HashSet<Move> {
    let single_move = Move {
        previous_commands: Vec::new(),
        speed: *starting_speed,
        cars_y: cars_starting_y.clone(),
        cars_x: 0,
        previous_moves: Vec::new(),
    };

    let start = Instant::now();

    let completed_moves = recursive_find_moves(
        &lanes,
        &min_cars_to_win,
        &single_move,
        &start,
        max_search_time_ms
    );

    completed_moves
}

fn breadth_first_search(
    lanes: &Vec<Vec<u8>>,
    min_cars_to_win: &i32,
    starting_speed: &i32,
    cars_starting_y: &Vec<i32>,
) -> HashSet<Move> {

    let mut completed_moves = HashSet::<Move>::new();
    let mut moves = HashSet::<Move>::from([
        Move {
            previous_commands: Vec::new(),
            speed: *starting_speed,
            cars_y: cars_starting_y.clone(),
            cars_x: 0,
            previous_moves: Vec::new(),
        }
    ]);

    while !moves.is_empty() {
        let temp_moves = moves;
        moves = HashSet::new();

        for single_move in &temp_moves {
            let valid_moves = calculate_next_moves(
                &lanes,
                single_move,
                &min_cars_to_win,
            );

            // println!("{:?}", valid_moves);

            for valid_move in valid_moves {
                if valid_move.cars_x == (lanes[0].len() + 1) as i32 {
                    completed_moves.insert(valid_move);
                } else {
                    moves.insert(valid_move);
                }
            }
        }

        // for single in &moves {
        //     println!("{:?}", *single);
        // }
        // println!("moves.len {} completed_moves.len() {}", moves.len(), completed_moves.len());
        //
        // let mut input_line = String::new();
        // io::stdin().read_line(&mut input_line).unwrap();
    }

    completed_moves
}

fn recursive_find_moves(
    lanes: &Vec<Vec<u8>>,
    min_cars_to_win: &i32,
    single_move: &Move,
    start_time: &Instant,
    max_search_time_ms: &u128
) -> HashSet<Move> {
    let mut completed_moves = HashSet::<Move>::new();

    let mut valid_moves = calculate_next_moves(
        &lanes,
        single_move,
        &min_cars_to_win,
    );

    valid_moves.sort_by(
        |a, b|
            b.previous_commands.last().expect("fail_b").partial_cmp(a.previous_commands.last().expect("fail_a")).unwrap()
    );

    // while !valid_moves.is_empty() {
    //     let next_move = valid_moves.pop().expect("failed pop");
    //
    //     if next_move.cars_x == (lanes[0].len() + 1) as i32 {
    //         completed_moves.push(next_move);
    //     } else {
    //         completed_moves = recursive_find_moves(
    //             lanes,
    //             min_cars_to_win,
    //             &next_move,
    //         )
    //     }
    //
    //     if !completed_moves.is_empty() {
    //         return completed_moves;
    //     }
    // }

    while !valid_moves.is_empty() {
        let next_move = valid_moves.pop().expect("failed pop");

        // let previous_commands = single_move.previous_commands.clone();
        if next_move.cars_x == (lanes[0].len() + 1) as i32 {
            completed_moves.insert(next_move);
        } else {
            completed_moves.extend(
                recursive_find_moves(
                    lanes,
                    min_cars_to_win,
                    &next_move,
                    start_time,
                    max_search_time_ms
                )
            );
        }

        if start_time.elapsed().as_millis() >= *max_search_time_ms {
            return completed_moves;
        }
        // if single_move.previous_commands.len() == 3 {
        //     println!("completed_moves.len() {} starting {:?}", completed_moves.len(), previous_commands);
        // }
    }

    completed_moves
}

fn calculate_next_moves(
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();

    if current_move.previous_commands.len() == 49 {
        return moves;
    }

    moves = insert_if_can_go_up(
        moves,
        lanes,
        current_move,
        min_cars_to_win,
    );

    moves = insert_if_can_go_down(
        moves,
        lanes,
        current_move,
        min_cars_to_win,
    );

    // moves = insert_if_can_wait(
    //     moves,
    //     lanes,
    //     current_move,
    //     min_cars_to_win,
    // );

    moves = insert_if_can_jump(
        moves,
        lanes,
        current_move,
        min_cars_to_win,
    );

    moves = insert_if_can_slow(
        moves,
        lanes,
        current_move,
        min_cars_to_win,
    );

    moves = insert_if_can_speed(
        moves,
        lanes,
        current_move,
        min_cars_to_win,
    );

    moves
}

fn insert_if_can_go_up(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    if current_move.speed == 0 {
        return moves;
    }

    let next_x = cmp::min(current_move.cars_x + current_move.speed, (lanes[0].len() + 1) as i32);

    let mut cars_y_clone = Vec::new();

    //if the car is already at the end of the track, there are no more obstacles possible
    if current_move.cars_x == lanes[0].len() as i32
        && next_x == (lanes[0].len() + 1) as i32 {
        cars_y_clone = current_move.cars_y.clone();
    } else {
        let next_index = cmp::min(next_x, (lanes[0].len() - 1) as i32);
        'y_check: for &y in &current_move.cars_y {
            //Cannot go up if a car is already at the top
            if y == 0 {
                return moves;
            }

            let next_y = y - 1;
            //Any holes between prev move and here must be taken into account.
            for i in current_move.cars_x..=next_index {
                if (i != next_index
                    && lanes[y as usize][i as usize] == b'0')
                    || lanes[next_y as usize][i as usize] == b'0'
                {
                    continue 'y_check;
                }
            }

            cars_y_clone.push(next_y);
        }
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push(MoveValues::UP);

    let mut previous_moves = current_move.previous_moves.clone();
    let mut move_clone = current_move.clone();
    move_clone.previous_moves.clear();
    previous_moves.push(move_clone);

    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: next_x,
            previous_moves,
        }
    );

    moves
}

fn insert_if_can_go_down(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move>
{
    if current_move.speed == 0 {
        return moves;
    }

    let next_x = cmp::min(current_move.cars_x + current_move.speed, (lanes[0].len() + 1) as i32);

    let mut cars_y_clone = Vec::<i32>::new();

    //if the car is already at the end of the track, there are no more obstacles possible
    if current_move.cars_x == lanes[0].len() as i32
        && next_x == (lanes[0].len() + 1) as i32 {
        cars_y_clone = current_move.cars_y.clone();
    } else {
        let next_index = cmp::min(next_x, (lanes[0].len() - 1) as i32);
        'y_check: for &y in &current_move.cars_y {
            //Cannot go up if a car is already at the top
            if y == 3 {
                return moves;
            }

            let next_y = y + 1;
            //Any holes between prev move and here must be taken into account.
            for i in current_move.cars_x..=next_index {
                if (i != next_index
                    && lanes[y as usize][i as usize] == b'0')
                    || lanes[next_y as usize][i as usize] == b'0'
                {
                    continue 'y_check;
                }
            }

            cars_y_clone.push(next_y);
        }
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push(MoveValues::DOWN);

    let mut previous_moves = current_move.previous_moves.clone();
    let mut move_clone = current_move.clone();
    move_clone.previous_moves.clear();
    previous_moves.push(move_clone);

    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: next_x,
            previous_moves,
        }
    );

    moves
}

fn insert_if_can_wait(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    if current_move.speed == 0 {
        return moves;
    }

    let next_x = cmp::min(current_move.cars_x + current_move.speed, (lanes[0].len() + 1) as i32);

    let mut cars_y_clone = Vec::<i32>::new();


    //if the car is already at the end of the track, there are no more obstacles possible
    if current_move.cars_x == lanes[0].len() as i32
        && next_x == (lanes[0].len() + 1) as i32 {
        cars_y_clone = current_move.cars_y.clone();
    } else {
        let next_index = cmp::min(next_x, (lanes[0].len() - 1) as i32);
        'y_check: for &y in &current_move.cars_y {
            //Any holes between prev move and here must be taken into account.
            for i in current_move.cars_x..=next_index {
                if lanes[y as usize][i as usize] == b'0'
                {
                    continue 'y_check;
                }
            }

            cars_y_clone.push(y);
        }
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push(MoveValues::WAIT);

    let mut previous_moves = current_move.previous_moves.clone();
    let mut move_clone = current_move.clone();
    move_clone.previous_moves.clear();
    previous_moves.push(move_clone);

    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: next_x,
            previous_moves,
        }
    );

    moves
}

fn insert_if_can_jump(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    if current_move.speed == 0 {
        return moves;
    }

    let next_x = cmp::min(current_move.cars_x + current_move.speed, (lanes[0].len() + 1) as i32);

    let mut cars_y_clone = Vec::<i32>::new();

    //if the car is already at the end of the track, there are no more obstacles possible
    if current_move.cars_x == lanes[0].len() as i32
        && next_x == (lanes[0].len() + 1) as i32 {
        cars_y_clone = current_move.cars_y.clone();
    } else {
        for &y in &current_move.cars_y {
            //Only the hole at the end of the jump needs to be taken into account.
            if next_x < lanes[0].len() as i32 && lanes[y as usize][next_x as usize] == b'0' {
                continue;
            }

            cars_y_clone.push(y);
        }
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push(MoveValues::JUMP);

    let mut previous_moves = current_move.previous_moves.clone();
    let mut move_clone = current_move.clone();
    move_clone.previous_moves.clear();
    previous_moves.push(move_clone);

    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: current_move.speed,
            cars_y: cars_y_clone,
            cars_x: next_x,
            previous_moves,
        }
    );

    moves
}

fn insert_if_can_slow(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    let next_speed = current_move.speed - 1;

    if next_speed <= 0 {
        return moves;
    }

    let next_x = cmp::min(current_move.cars_x + next_speed, (lanes[0].len() + 1) as i32);

    let mut cars_y_clone = Vec::<i32>::new();

    //if the car is already at the end of the track, there are no more obstacles possible
    if current_move.cars_x == lanes[0].len() as i32
        && next_x == (lanes[0].len() + 1) as i32 {
        cars_y_clone = current_move.cars_y.clone();
    } else {
        let next_index = cmp::min(next_x, (lanes[0].len() - 1) as i32);
        'y_check: for &y in &current_move.cars_y {
            //Any holes between prev move and here must be taken into account.
            for i in current_move.cars_x..=next_index {
                if lanes[y as usize][i as usize] == b'0'
                {
                    continue 'y_check;
                }
            }

            cars_y_clone.push(y);
        }
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push(MoveValues::SLOW);

    let mut previous_moves = current_move.previous_moves.clone();
    let mut move_clone = current_move.clone();
    move_clone.previous_moves.clear();
    previous_moves.push(move_clone);

    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: next_speed,
            cars_y: cars_y_clone,
            cars_x: next_x,
            previous_moves,
        }
    );

    moves
}

fn insert_if_can_speed(
    mut moves: Vec<Move>,
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    let next_speed = current_move.speed + 1;

    if next_speed <= 0 {
        return moves;
    }

    let next_x = cmp::min(current_move.cars_x + next_speed, (lanes[0].len() + 1) as i32);

    let mut cars_y_clone = Vec::<i32>::new();

    //if the car is already at the end of the track, there are no more obstacles possible
    if current_move.cars_x == lanes[0].len() as i32
        && next_x == (lanes[0].len() + 1) as i32 {
        cars_y_clone = current_move.cars_y.clone();
    } else {
        let next_index = cmp::min(next_x, (lanes[0].len() - 1) as i32);
        'y_check: for &y in &current_move.cars_y {
            //Any holes between prev move and here must be taken into account.
            for i in current_move.cars_x..=next_index {
                if lanes[y as usize][i as usize] == b'0'
                {
                    continue 'y_check;
                }
            }

            cars_y_clone.push(y);
        }
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push(MoveValues::SPEED);

    let mut previous_moves = current_move.previous_moves.clone();
    let mut move_clone = current_move.clone();
    move_clone.previous_moves.clear();
    previous_moves.push(move_clone);

    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: next_speed,
            cars_y: cars_y_clone,
            cars_x: next_x,
            previous_moves,
        }
    );

    moves
}












/*
// it has to be calculate on every square the car could possibly have come from
fn main() {

    // 1 2 3 3 4 5 6 7
    //                     10                  20                   30
    // 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9
    //                                                           |


    let lanes: Vec<Vec<u8>> = Vec::from([
        "..............................".as_bytes().to_vec(),
        "..............................".as_bytes().to_vec(),
        "...........0..................".as_bytes().to_vec(),
        "..............................".as_bytes().to_vec(),
    ]);

    let starting_speed = 0;
    let min_cars_to_win = 1;
    let cars_starting_y = Vec::<i32>::from([2]);


    let final_moves = calculate_moves(
        &lanes,
        &starting_speed,
        &min_cars_to_win,
        cars_starting_y,
    );

    //TODO: need to sort results

    for final_move in final_moves {
        println!("final_move: {:?}", final_move);
    }
}

fn calculate_moves(
    lanes: &Vec<Vec<u8>>,
    starting_speed: &i32,
    min_cars_to_win: &i32,
    cars_starting_y: Vec<i32>,
) -> HashSet<Move> {
    let cars_y_possibilities: Vec<Vec<i32>> = Vec::from([Vec::from([1])]);

    // let num_cars = cars_starting_y.len() as i32;
    // let mut cars_y_possibilities: Vec<Vec<i32>> = Vec::from([cars_starting_y]);
    //
    // //TODO: need to calculate all possible y combos for cars
    // // move them all down, then move them all up
    // // need every possibility for the minimum cars surviving
    // for i in 1..=(num_cars - *min_cars_to_win) {
    //
    // }

    //TODO: there is a problem here, all of the below commands were returned, none of them are valid
    // ["JUMP", "UP", "SPEED", "SPEED", "WAIT", "SPEED", "SPEED", "SPEED"], speed: 1, cars_y: [2], cars_x: 0
    // ["JUMP", "SLOW", "UP", "JUMP", "SPEED", "SPEED", "SPEED", "SPEED"], speed: 1, cars_y: [2], cars_x: 0
    // ["SPEED", "UP", "SPEED", "JUMP", "SPEED", "SPEED", "SPEED", "SPEED"], speed: 1, cars_y: [2], cars_x: 0
    // It doesn't seem to be properly handling holes, these should have fallen in
    // It doesn't seem to be calculating speed properly, the SLOW value above should obviously not be possible
    //   Maybe I need to change the way SLOW and SPEED calculate largest and smallest speed, if the user uses SLOW as the last move for example it should change something
    //   It probably has something to do w/ the starting x position being back too far
    //   In order for it to be the full 7, it probably (?) needs to be speed every time, if it is a slow then I think 6 is the max?


    //TODO: there is the generic one of
    // "SPEED"
    // "SPEED"
    // "SPEED"
    // "SPEED"
    // "UP"
    // "SPEED"
    // "SPEED"
    // "SPEED" <- first command
    // or
    // "SPEED"
    // "SPEED"
    // "SPEED"
    // "JUMP"
    // "SPEED"
    // "SPEED"
    // "SPEED"
    // "SPEED" <- first command

    let highest_possible_speed = calculate_highest_possible_speed(
        *starting_speed as f32,
        lanes[0].len() as f32,
    );

    let lowest_possible_speed = calculate_lowest_possible_speed(
        *starting_speed as f32,
        lanes[0].len() as f32,
    );

    let mut finishes_moves: HashSet<Move> = HashSet::new();
    let mut in_progress_moves: HashSet<Move> = HashSet::new();

    for speed in lowest_possible_speed..=highest_possible_speed {
        println!("speed_loop");
        for difference_on_x in 1..=speed {
            println!("difference_loop");
            //the final index is the end point, so -1 will get me to that point
            let x_pos = lanes[0].len() as i32 - 2 + difference_on_x;

            if x_pos < 0 {
                continue;
            }

            for cars_y in &cars_y_possibilities {
                println!("cars_y loop");
                let current_move = Move {
                    previous_commands: Vec::new(),
                    speed,
                    cars_y: cars_y.clone(),
                    cars_x: x_pos,
                };

                let returned_moves = get_previous_valid_moves(
                    lanes,
                    &current_move,
                    starting_speed,
                    min_cars_to_win,
                );

                for returned_move in returned_moves {
                    if returned_move.cars_y == cars_starting_y
                        && (returned_move.speed == *starting_speed
                            || (returned_move.speed == *starting_speed + 1
                                && returned_move.previous_commands.last().expect("failed") == "SPEED")
                        )
                        && returned_move.cars_x == 0 {
                        finishes_moves.insert(returned_move);
                    } else if returned_move.speed > 0 && returned_move.cars_x > 0 {
                        in_progress_moves.insert(returned_move);
                    }
                }
            }
        }
    }

    for mov in &in_progress_moves {
        println!("{:?}", *mov);
    }
    println!("in_progress_moves.len(): {}", in_progress_moves.len());
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line);

    while finishes_moves.is_empty() {
        let checking_moves = in_progress_moves;
        in_progress_moves = HashSet::new();
        for curr_move in checking_moves {
            //println!("curr_move loop");
            let returned_moves = get_previous_valid_moves(
                lanes,
                &curr_move,
                starting_speed,
                min_cars_to_win,
            );

            for returned_move in returned_moves {
                if returned_move.cars_y == cars_starting_y
                    && (returned_move.speed == *starting_speed
                        || (returned_move.speed == *starting_speed + 1
                            && returned_move.previous_commands.last().expect("failed") == "SPEED")
                    ) //the starting move can be SPEED which technically changes the 'starting speed'
                    && returned_move.cars_x == 0 {
                    finishes_moves.insert(returned_move);
                } else if returned_move.speed > 0 && returned_move.cars_x > 0 {
                    in_progress_moves.insert(returned_move);
                }
            }
        }

        // for mov in &in_progress_moves {
        //     let mut num_speed = 0;
        //     for cmd in &mov.previous_commands {
        //         if *cmd == "SPEED".to_string() {
        //             num_speed += 1;
        //         }
        //     }
        //     if num_speed >= (mov.previous_commands.len() - 1) {
        //         println!("{:?}", *mov);
        //     }
        // }
        // println!("in_progress_moves.len(): {}", in_progress_moves.len());
        // println!("finishes_moves.len() {}", finishes_moves.len());
        // io::stdin().read_line(&mut input_line);
    }

    finishes_moves
}

fn get_previous_valid_moves(
    lanes: &Vec<Vec<u8>>,
    current_move: &Move,
    starting_speed: &i32,
    min_cars_to_win: &i32,
) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();

    if current_move.previous_commands.len() == 49 {
        return moves;
    }

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

    let current_x = calculate_current_x(
        lanes.len(),
        current_move.cars_x,
    );

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Cannot go up if a car is already at the top
        if y == 0 {
            return moves;
        }

        let prev_y = y - 1;
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_x {
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
        &current_move.speed,
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
            cars_x: prev_x,
        }
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

    let current_x = calculate_current_x(
        lanes.len(),
        current_move.cars_x,
    );

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Cannot go up if a car is already at the top
        if y == 3 {
            return moves;
        }

        let prev_y = y + 1;
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_x {
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
        &current_move.speed,
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
            cars_x: prev_x,
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

    let current_x = calculate_current_x(
        lanes.len(),
        current_move.cars_x,
    );

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_x {
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
        &current_move.speed,
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
            cars_x: prev_x,
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
        &current_move.speed,
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
            cars_x: prev_x,
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

    let current_x = calculate_current_x(
        lanes.len(),
        current_move.cars_x,
    );

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &new_speed,
    );

    if !valid_move {
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_x {
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
            cars_x: prev_x,
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

    let compare = Vec::<String>::from(["SPEED".to_string(), "SPEED".to_string()]);

    // if new_speed == 0 {
    //     if current_move.previous_commands == compare {
    //         println!("failed at new_speed");
    //     }
    //     return moves;
    // }

    if (lanes[0].len() as i32 - 1) <= prev_x {
        if current_move.previous_commands == compare {
            println!("failed at lanes[0].len() - 1");
        }
        return moves;
    }

    //If before start.
    if prev_x < 0 {
        if current_move.previous_commands == compare {
            println!("failed at prev_x");
        }
        return moves;
    }

    let current_x = calculate_current_x(
        lanes.len(),
        current_move.cars_x,
    );

    let valid_move = speed_is_valid(
        &prev_x,
        starting_speed,
        &new_speed,
    );

    if !valid_move {
        if current_move.previous_commands == compare {
            println!("failed at valid_move");
        }
        return moves;
    }

    let mut cars_y_clone = Vec::<i32>::new();

    'y_check: for &y in &current_move.cars_y {
        //Any holes between prev move and here must be taken into account.
        for i in prev_x..current_x {
            if lanes[y as usize][i as usize] == b'0'
            {
                continue 'y_check;
            }
        }

        cars_y_clone.push(y);
    }

    //If too many cars fell in holes.
    if cars_y_clone.len() < *min_cars_to_win as usize {
        if current_move.previous_commands == compare {
            println!("failed at cars_y_clone.len()");
        }
        return moves;
    }

    let mut commands_clone = current_move.previous_commands.clone();
    commands_clone.push("SPEED".to_string());
    moves.push(
        Move {
            previous_commands: commands_clone,
            speed: new_speed,
            cars_y: cars_y_clone,
            cars_x: prev_x,
        }
    );

    moves
}


fn speed_is_valid(
    x_pos: &i32,
    starting_speed: &i32,
    current_speed: &i32,
) -> bool {
    let x_pos_f = *x_pos as f32;
    let starting_speed_f = *starting_speed as f32;

    let highest_possible_speed = calculate_highest_possible_speed(
        starting_speed_f,
        x_pos_f + 1.0,
    );

    let lowest_possible_speed = calculate_lowest_possible_speed(
        starting_speed_f,
        x_pos_f + 1.0,
    );

    if *current_speed < lowest_possible_speed || highest_possible_speed < *current_speed {
        false
    } else {
        true
    }
}

fn calculate_highest_possible_speed(
    starting_speed: f32,
    distance: f32,
) -> i32 {
    //sqrt(start_speed^2 + 2 * track_len)
    ((starting_speed * starting_speed) + 2.0 * (distance + 1.0)).sqrt() as i32
}

fn calculate_lowest_possible_speed(
    starting_speed: f32,
    distance: f32,
) -> i32 {
    //sqrt(start_speed^2 - 2 * track_len)
    let lowest_inner_part = (starting_speed * starting_speed) - 2.0 * (distance);
    if lowest_inner_part <= 1.0 {
        lowest_inner_part.sqrt() as i32
    } else {
        0
    }
}

fn calculate_current_x(
    lanes_len: usize,
    cars_x: i32,
) -> i32 {
    //must take into consideration that the final space could be a hold
    if cars_x >= lanes_len as i32 {
        lanes_len as i32
    } else {
        cars_x
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
*/