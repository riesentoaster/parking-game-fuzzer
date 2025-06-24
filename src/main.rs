use num_traits::{CheckedMul, One, Unsigned, Zero};
use parking_game::{Car, Orientation, Position, State};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt::Debug;
use std::ops::{AddAssign, Sub};
use std::{env, fs};

/// Parses a map with the following rules:
/// 1. The board is 6x6.
/// 2. Empty spaces are denoted with `.`.
/// 3. The car which must be moved to the objective is referenced with `o`. This will be index 1.
/// 4. All other cars are uniquely named.
/// 5. All cars are at least length 2.
///
/// Any map not following this pattern is not guaranteed to be parsed correctly.
fn parse_map<T>(map: &str) -> State<T>
where
    T: TryFrom<usize>
        + PartialEq
        + One
        + AddAssign
        + Sub
        + Copy
        + CheckedMul
        + Zero
        + Ord
        + Unsigned
        + Debug,
    T::Error: Debug,
    usize: From<T>,
{
    let map = map.trim_ascii();
    let rows = map.lines().count().try_into().unwrap();
    let cols = map.lines().next().unwrap().len().try_into().unwrap();

    let mut cars: HashMap<char, (Position<T>, Orientation, T)> = HashMap::new();

    let mut prev = None;
    for (ridx, row) in map.lines().enumerate() {
        let ridx = ridx.try_into().unwrap();
        for (cidx, col) in row.chars().enumerate() {
            let cidx: T = cidx.try_into().unwrap();
            match (prev, col) {
                (Some(car), next) => {
                    match cars.entry(car) {
                        Entry::Occupied(mut e) => {
                            let entry = e.get_mut();
                            assert!(
                                entry.0.row() == &ridx || entry.0.column() == &(cidx - T::one())
                            );
                            entry.2 += T::one();
                        }
                        Entry::Vacant(e) => {
                            if car == next {
                                // same car: we are in the same row, init with left-right
                                e.insert((
                                    (ridx, cidx - T::one()).into(),
                                    Orientation::LeftRight,
                                    T::one(),
                                ));
                            } else {
                                // different car: different row, init with up-down
                                e.insert((
                                    (ridx, cidx - T::one()).into(),
                                    Orientation::UpDown,
                                    T::one(),
                                ));
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
                    assert!(entry.0.row() == &ridx || entry.0.column() == &(cols - T::one()));
                    entry.2 += T::one();
                }
                Entry::Vacant(e) => {
                    // this has to be up-down orientation: we haven't seen it earlier
                    e.insert((
                        (ridx, cols - T::one()).into(),
                        Orientation::UpDown,
                        T::one(),
                    ));
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
    // adjust u8 to u16 as necessary
    let init = parse_map::<u8>(&fs::read_to_string(path).unwrap());

    println!("Attempting to solve:");
    println!("{}", init.board().unwrap());

    todo!("Left as an exercise to the reader.")
}
