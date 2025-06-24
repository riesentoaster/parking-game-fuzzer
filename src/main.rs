use parking_game::{Car, Orientation, Position, State};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::{env, fs};

/// Parses a map with the following rules:
/// 1. The board is 6x6.
/// 2. Empty spaces are denoted with `.`.
/// 3. The car which must be moved to the objective is referenced with `o`.
/// 4. All other cars are uniquely named.
/// 5. All cars are at least length 2.
///
/// Any map not following this pattern is not guaranteed to be parsed correctly.
fn parse_map(map: &str) -> State<u8> {
    let map = map.trim_ascii();
    let rows = map.lines().count() as u8;
    let cols = map.lines().next().unwrap().len() as u8;

    assert_eq!((6, 6), (rows, cols));

    let mut cars: HashMap<char, (Position<u8>, Orientation, u8)> = HashMap::new();

    let mut prev = None;
    for (ridx, row) in map.lines().enumerate() {
        let ridx = ridx as u8;
        for (cidx, col) in row.chars().enumerate() {
            let cidx = cidx as u8;
            match (prev, col) {
                (Some(car), next) => {
                    match cars.entry(car) {
                        Entry::Occupied(mut e) => {
                            let entry = e.get_mut();
                            assert!(entry.0.row() == &ridx || entry.0.column() == &(cidx - 1));
                            entry.2 += 1;
                        }
                        Entry::Vacant(e) => {
                            if car == next {
                                // same car: we are in the same row, init with left-right
                                e.insert(((ridx, cidx - 1).into(), Orientation::LeftRight, 1u8));
                            } else {
                                // different car: different row, init with up-down
                                e.insert(((ridx, cidx - 1).into(), Orientation::UpDown, 1u8));
                            }
                        }
                    }
                    prev = if next == '.' { None } else { Some(next) };
                }
                (None, '.') => {
                    // do nothing
                }
                (None, next) => {
                    prev = Some(next);
                }
            }
        }

        if let Some(car) = prev.take() {
            match cars.entry(car) {
                Entry::Occupied(mut e) => {
                    let entry = e.get_mut();
                    assert!(entry.0.row() == &ridx || entry.0.column() == &(cols - 1));
                    entry.2 += 1;
                }
                Entry::Vacant(e) => {
                    // this has to be up-down orientation: we haven't seen it earlier
                    e.insert(((ridx, cols - 1).into(), Orientation::UpDown, 1u8));
                }
            }
        }
    }

    let mut state = State::empty((rows, cols)).unwrap();
    let mut inserted = VecDeque::new();
    for (name, (pos, orientation, len)) in cars {
        if name == 'o' {
            inserted.push_front((pos, Car::new(len, orientation).unwrap()))
        } else {
            inserted.push_back((pos, Car::new(len, orientation).unwrap()));
        }
    }

    let mut board = state.board_mut().unwrap();
    for (position, car) in inserted {
        board.add_car(position, car).unwrap();
    }
    drop(board);

    state
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args_os()
        .nth(1)
        .expect("Provide the path to the desired map.");
    let init = parse_map(&fs::read_to_string(path).unwrap());

    println!("Attempting to solve:");
    println!("{}", init.board().unwrap());

    todo!("Left as an exercise to the reader.")
}
