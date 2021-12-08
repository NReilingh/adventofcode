pub fn play_bingo(input: Vec<String>) -> (u32, u32) {
    (0, 0)
}

#[derive(Clone, Copy, Debug)]
enum BingoSpace {
    Marked,
    Unmarked(u32),
}
use BingoSpace::{Marked, Unmarked};

struct BingoBoard {
    board: [BingoSpace; 25],
    plays: u32,
}

impl From<&[u32]> for BingoBoard {
    fn from(grid: &[u32]) -> Self {
        BingoBoard {
            board: grid.iter()
                .map(|item| Unmarked(*item))
                .collect::<Vec<BingoSpace>>().try_into().unwrap(),
            plays: 0,
        }
    }
}

impl From<Vec<BingoSpace>> for BingoBoard {
    fn from(grid: Vec<BingoSpace>) -> Self {
        BingoBoard {
            board: grid.try_into().unwrap(),
            plays: 0,
        }
    }
}

impl BingoBoard {
    fn play(&mut self, draw: &u32) -> Option<&u32> {
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

        // search each axis for all Marked spaces and return Some(draw) if found

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

#[cfg(test)]
mod function_tests {
    use super::*;

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
}
