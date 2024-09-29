/* This module has functions for updating metadata for a Chessboard struct */
use crate::chess::{BoardPiece, ChessBoard, MetaData, Move, Pieces};

impl ChessBoard {
    pub fn update_meta_data(&mut self, move_to_make: Move) {
        match move_to_make.meta_data {
            MetaData::Move => {}

            MetaData::Capture => {
                self.reset_half_move_clock();
                self.set_no_en_passant();
            }

            MetaData::Castling => {}

            MetaData::EnPassant => {}

            MetaData::Promotion(_) => {
                self.reset_half_move_clock();
                self.set_no_en_passant();
            }
        }
        self.update_fullmove_counter();
    }

    fn update_fullmove_counter(&mut self) {
        if !self.white_is_side_to_move {
            self.fullmove_counter += 1;
        }
    }

    fn update_side_to_move(&mut self) {
        self.white_is_side_to_move = !self.white_is_side_to_move;
    }

    fn set_no_en_passant(&mut self) {
        if self.en_passant_target_square.is_some() {
            self.en_passant_target_square = None;
        }
    }

    fn increment_half_move_clock(&mut self) {
        self.halfmove_clock += 1;
    }
    fn reset_half_move_clock(&mut self) {
        self.halfmove_clock = 0;
    }
}

pub fn update_move(chessboard: &mut ChessBoard, piece: &BoardPiece, move_to_make: &Move) {}

pub fn update_capture(chessboard: &mut ChessBoard) {}

pub fn update_castle(chessboard: &mut ChessBoard) {}

pub fn update_en_passant(chessboard: &mut ChessBoard) {}

pub fn update_pawn_promotion(chessboard: &mut ChessBoard) {}
