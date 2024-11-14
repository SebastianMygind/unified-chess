const VALID_POSITION_CHARS: [char; 12] =
    ['k', 'q', 'r', 'b', 'n', 'p', 'K', 'Q', 'R', 'B', 'N', 'P'];

const VALID_FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

///
/// FEN validation
///
pub fn is_position_valid(position: &str) -> bool {
    let mut ranks = position.split('/');
    let mut rank: u32 = 0;

    while let Some(rank_string) = ranks.next() {
        let mut file = 0;

        for char in rank_string.chars() {
            match char.to_digit(10) {
                Some(digit) => {
                    file += digit;
                }

                None => {
                    if !VALID_POSITION_CHARS.contains(&char) {
                        return false;
                    }
                    file += 1;
                }
            }
        }
        if file != 8 {
            return false;
        }
        rank += 1;
    }

    if rank != 8 {
        return false;
    }
    true
}

pub fn is_side_to_move_valid(side_to_move: &str) -> bool {
    let mut chars = side_to_move.chars();

    match chars.next() {
        Some(c) => {
            if !(c == 'w' || c == 'b') {
                return false;
            }
        }
        None => return false,
    };

    if chars.next().is_some() {
        return false;
    }

    true
}

pub fn is_castling_valid(castling_ability: &str) -> bool {
    let mut chars = castling_ability.chars();
    let mut has_no_ability = false;
    let mut char_occurences: [u32; 4] = [0; 4]; //WKing, WQueen, BKing, BQueen

    while let Some(c) = chars.next() {
        match c {
            'K' => char_occurences[0] += 1,
            'Q' => char_occurences[1] += 1,
            'k' => char_occurences[2] += 1,
            'q' => char_occurences[3] += 1,
            '-' => {
                if !has_no_ability {
                    has_no_ability = true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    if has_no_ability {
        for count in char_occurences {
            if count != 0 {
                return false;
            }
        }
    } else {
        for count in char_occurences {
            if !(count == 1 || count == 0) {
                return false;
            }
        }
    }
    true
}

pub fn is_en_passant_valid(en_passant: &str) -> bool {
    let mut chars = en_passant.chars();
    let mut no_en_passant = false;

    if let Some(char) = chars.next() {
        match char {
            '-' => no_en_passant = true,
            _ => {
                if !VALID_FILES.contains(&char) {
                    return false;
                }
            }
        }
    } else {
        return false;
    }

    let rank_option = chars.next();

    if no_en_passant && rank_option.is_some() {
        return false;
    }

    if let Some(rank_char) = rank_option {
        let digit = match rank_char.to_digit(10) {
            Some(digit) => digit,
            None => return false,
        };
        if digit != 3 && digit != 6 {
            return false;
        }
    }
    true
}

pub fn is_half_move_valid(half_move: &str) -> bool {
    match half_move.parse::<u32>() {
        Ok(half_move) => {
            if half_move > 50 {
                return false;
            }
        }
        Err(_) => return false,
    }
    true
}

pub fn is_move_counter_valid(move_counter: &str) -> bool {
    move_counter.parse::<u32>().is_ok()
}

///
/// FEN parsing
///

///
/// FEN unit tests
///

// FEN validation tests
#[cfg(test)]
mod tests {
    use super::*;

    // Piece position tests

    #[test]
    fn test_is_position_valid1() {
        assert_eq!(
            is_position_valid("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            true
        );
    }

    #[test]
    fn test_is_position_valid2() {
        assert_eq!(
            is_position_valid("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R"),
            true
        );
    }

    #[test]
    fn test_is_position_valid3() {
        assert_eq!(
            is_position_valid("rnbhkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            false
        )
    }

    #[test]
    fn test_is_position_valid4() {
        assert_eq!(
            is_position_valid("rnbqkbnr/pppppppp/8/7/8/8/PPPPPPPP/RNBQKBNR"),
            false
        );
    }

    #[test]
    fn test_is_position_valid5() {
        assert_eq!(
            is_position_valid("rnbqkbnr/pppppppp/8/8/9/8/PPPPPPPP/RNBQKBNR"),
            false
        );
    }

    // Side to move tests

    #[test]
    fn test_is_side_to_move_valid1() {
        assert_eq!(is_side_to_move_valid("w"), true);
    }

    #[test]
    fn test_is_side_to_move_valid2() {
        assert_eq!(is_side_to_move_valid("b"), true);
    }

    #[test]
    fn test_is_side_to_move_valid3() {
        assert_eq!(is_side_to_move_valid("wb"), false);
    }

    #[test]
    fn test_is_side_to_move_valid4() {
        assert_eq!(is_side_to_move_valid("h"), false);
    }

    // Castling ability tests

    #[test]
    fn test_castling1() {
        assert_eq!(is_castling_valid("KQkq"), true);
    }

    #[test]
    fn test_castling2() {
        assert_eq!(is_castling_valid("Qkq"), true);
    }

    #[test]
    fn test_castling3() {
        assert_eq!(is_castling_valid("K"), true);
    }

    #[test]
    fn test_castling4() {
        assert_eq!(is_castling_valid("-"), true);
    }

    #[test]
    fn test_castling5() {
        assert_eq!(is_castling_valid("qqq"), false);
    }

    #[test]
    fn test_castling6() {
        assert_eq!(is_castling_valid("k-"), false);
    }

    #[test]
    fn test_castling7() {
        assert_eq!(is_castling_valid("--"), false);
    }

    // En passant tests

    #[test]
    fn test_en_passant1() {
        assert_eq!(is_en_passant_valid("-"), true);
    }

    #[test]
    fn test_en_passant2() {
        assert_eq!(is_en_passant_valid("e3"), true);
    }

    #[test]
    fn test_en_passant3() {
        assert_eq!(is_en_passant_valid("f6"), true);
    }

    #[test]
    fn test_en_passant4() {
        assert_eq!(is_en_passant_valid("e4"), false);
    }

    #[test]
    fn test_en_passant5() {
        assert_eq!(is_en_passant_valid("-e6"), false);
    }

    // Half move counter test

    #[test]
    fn half_move1() {
        assert_eq!(is_half_move_valid("20"), true);
    }

    #[test]
    fn half_move2() {
        assert_eq!(is_half_move_valid("50"), true);
    }

    #[test]
    fn half_move3() {
        assert_eq!(is_half_move_valid("51"), false);
    }

    #[test]
    fn half_move4() {
        assert_eq!(is_half_move_valid("hej"), false);
    }

    #[test]
    fn half_move5() {
        assert_eq!(is_half_move_valid("5ed"), false);
    }

    // Move counter tests

    #[test]
    fn move_counter1() {
        assert_eq!(is_move_counter_valid("1"), true);
    }

    #[test]
    fn move_counter2() {
        assert_eq!(is_move_counter_valid("43"), true);
    }

    #[test]
    fn move_counter3() {
        assert_eq!(is_move_counter_valid("12df"), false);
    }

    #[test]
    fn move_counter4() {
        assert_eq!(is_move_counter_valid("-4"), false);
    }
}
