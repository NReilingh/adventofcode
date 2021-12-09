pub fn play_bingo(input: Vec<String>) -> (u32, u32) {
    // Parse input (strip the first line, split on double-newlines,
    // Join with space, then split on whitespace to parse as ints.
    let mut input_lines = input.iter();

    let draws: Vec<u32> = input_lines.next().unwrap().split(',')
        .map(|s| s.parse().unwrap()).collect();

    // Need to do this silly shadowing pattern because of E0716
    let grids = input_lines.cloned().collect::<Vec<String>>().join(" ");
    let grids = grids.split_whitespace().collect::<Vec<&str>>();

    // Create a vector of BingoBoards and instantiate them
    let mut games: Vec<BingoBoard> = grids.chunks(25)
        .map(|chunk|
            BingoBoard::from(chunk.iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>())
        ).collect();

    // Loop through the draws, loop through the boards until a winning draw
    // Then score the winning board
    let (mut first, mut last) = (0u32, 0u32);
    for draw in &draws {
        for game in &mut games.iter_mut().filter(|i| !i.won) {
            if let Some(winner) = game.play(draw) {
                if first == 0 {
                    first = game.score(winner);
                }
                last = game.score(winner);
                // Remove this game from games
                game.won = true;
            }
        }
    };

    (first, last)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BingoSpace {
    Marked,
    Unmarked(u32),
}
use BingoSpace::{Marked, Unmarked};

struct BingoBoard {
    board: [BingoSpace; 25],
    plays: u32,
    won: bool,
}

impl From<&[u32; 25]> for BingoBoard {
    fn from(grid: &[u32; 25]) -> Self {
        BingoBoard {
            board: grid.iter()
                .map(|item| Unmarked(*item))
                .collect::<Vec<BingoSpace>>().try_into().unwrap(),
            plays: 0,
            won: false,
        }
    }
}

impl From<Vec<u32>> for BingoBoard {
    fn from(grid: Vec<u32>) -> Self {
        BingoBoard {
            board: grid.iter()
                .map(|item| Unmarked(*item))
                .collect::<Vec<BingoSpace>>().try_into().unwrap(),
            plays: 0,
            won: false,
        }
    }
}

impl From<Vec<BingoSpace>> for BingoBoard {
    fn from(grid: Vec<BingoSpace>) -> Self {
        BingoBoard {
            board: grid.try_into().unwrap(),
            plays: 0,
            won: false,
        }
    }
}

use std::collections::HashSet;
impl BingoBoard {
    fn play(&mut self, draw: &u32) -> Option<u32> {
        // Play this board with the draw by mutating the board
        // for any marked positions, and also check to see if that caused us
        // to win by searching the x and y axes of the marked positions
        // and returning true if the board has won.
        //
        // Note that it is possible for one draw to mark multiple positions,
        // and for those positions to have a shared axis.
        //
        // So, we play the draw by marking any positions found
        // and returning those positions, then derive the unique search axes
        // for those positions, then check the search axes.
        //
        // Maybe we should also keep an internal count of the number of
        // draws that have been played so that we can skip searching for a win
        // before the fifth draw has been played.
        self.plays += 1;

        let mut found = Vec::new();

        for (i, space) in self.board.iter_mut().enumerate() {
            if let Unmarked(number) = space {
                if number == draw {
                    found.push(i);
                    *space = Marked;
                }
            }
        }

        if self.plays <5 {
            return None;
        }

        // derive axes for each found choice and dedupe them
        let mut axes = HashSet::new();
        for i in found {
            axes.insert(Axis::Horizontal((i / 5).try_into().unwrap()));
            axes.insert(Axis::Vertical((i % 5).try_into().unwrap()));
        }

        // search each axis for all Marked spaces and return Some(draw) if found
        for axis in axes {
            let all_marked = self.board.iter().enumerate()
                .filter(|(i, _)| {
                    match axis {
                        Axis::Horizontal(y) => i / 5 == y.into(),
                        Axis::Vertical(x) => i % 5 == x.into(),
                    }
                }).all(|(_, space)| *space == Marked);
            if all_marked {
                return Some(*draw);
            }
        }

        None
    }

    fn score(&self, winning_number: u32) -> u32 {
        winning_number * self.board.iter().filter_map(|space| {
            match space {
                Marked => None,
                Unmarked(score) => Some(score),
            }
        }).sum::<u32>()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Axis {
    Horizontal(u8),
    Vertical(u8),
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn play_airball() {
        let mut board = BingoBoard::from(&[7u32; 25]);
        board.plays = 5;
        assert_eq!(None, board.play(&8));
    }

    #[test]
    fn play_nothing_but_net() {
        let mut board = BingoBoard::from(&[7u32; 25]);
        board.plays = 4;
        assert_eq!(Some(7), board.play(&7));
    }

    #[test]
    fn play_vertical_win() {
        let mut board = BingoBoard::from(vec![
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Marked,
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Marked,
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(7),
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Marked,
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Marked,
        ]);
        board.plays = 4;
        assert_eq!(Some(7), board.play(&7));
    }

    #[test]
    fn play_horizontal_win() {
        let mut board = BingoBoard::from(vec![
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3),
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3),
            Marked, Unmarked(7), Marked, Unmarked(7), Marked,
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3),
            Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3), Unmarked(3),
        ]);
        board.plays = 4;
        assert_eq!(Some(7), board.play(&7));
    }

    #[test]
    fn play_game() {
        let mut board = BingoBoard::from(&[
             1u32, 2,  3,  4,  5,
             6,    7,  8,  9, 10,
            11,   12, 13, 14, 15,
            16,   17, 18, 19, 20,
            21,   22, 23, 24, 25,
        ]);
        assert_eq!(None, board.play(&2));
        assert_eq!(None, board.play(&22));
        assert_eq!(None, board.play(&12));
        assert_eq!(None, board.play(&17));
        assert_eq!(Some(7), board.play(&7));
    }

    #[test]
    fn board_score() {
        let mut grid = vec![Marked; 5];
        let mut spaces = vec![Unmarked(3); 20];
        grid.append(& mut spaces);
        let board = BingoBoard::from(grid);

        assert_eq!(300, board.score(5));
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn winning_bingo_board() {
        let input = read_input("../testinputs/04.txt");
        let (score, _) = play_bingo(input);
        assert_eq!(4512, score);
    }

    #[test]
    fn losingest_bingo_board() {
        let input = read_input("../testinputs/04.txt");
        let (_, score) = play_bingo(input);
        assert_eq!(1924, score);
    }
}
