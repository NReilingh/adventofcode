pub fn play_bingo(input: Vec<String>) -> (u32, u32) {
    (0, 0)
}

#[derive(Clone, Copy)]
enum BingoSpace {
    Marked,
    Unmarked(u32),
}

struct BingoBoard {
    board: [BingoSpace; 25],
    plays: u32,
}

impl BingoBoard {
    fn new(grid: &Vec<u32>) -> BingoBoard {
        BingoBoard {
            board: [BingoSpace::Unmarked(0); 25],
            plays: 0,
        }
    }

    fn play(&mut self, draw: u32) -> bool {
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
        false
    }

    fn score(&self, winning_number: u32) -> u32 {
        winning_number * self.board.iter().filter_map(|space| {
            match space {
                BingoSpace::Marked => None,
                BingoSpace::Unmarked(score) => Some(score),
            }
        }).sum::<u32>()
    }
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn board_score() {
        
        let board = BingoBoard {
            board: [
                BingoSpace::Marked, BingoSpace::Marked, BingoSpace::Marked, BingoSpace::Marked, BingoSpace::Marked,
                BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),
                BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),
                BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),
                BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),BingoSpace::Unmarked(3),
            ],
            plays: 0,
        };
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
