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

fn main() {

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

    let start = Instant::now();

    // let completed_moves = breadth_first_search(
    //     &lanes,
    //     &min_cars_to_win,
    //     &starting_speed,
    //     &cars_starting_y,
    // );


    let max_search_time_ms = 140;
    let completed_moves = depth_first_search(
        &lanes,
        &min_cars_to_win,
        &starting_speed,
        &cars_starting_y,
        &max_search_time_ms
    );

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

    let duration = start.elapsed();
    println!("Run time: {:?}", duration);

    println!("{:?}", instructions);

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