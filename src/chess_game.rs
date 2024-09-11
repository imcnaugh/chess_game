use crate::chess_piece::{ChessPiece, PieceType};
use crate::{ChessBoard, Color};

pub struct ChessGame {
    pub board: ChessBoard,
    current_turn: Color,
    turn_number: u32,
}

impl ChessGame {
    pub fn new() -> ChessGame {
        let board = ChessGame::create_board_with_starting_position();

        ChessGame {
            board,
            current_turn: Color::White,
            turn_number: 1,
        }
    }

    fn change_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn create_board_with_starting_position() -> ChessBoard {
        let mut board = ChessBoard::new();

        for i in 0..8 {
            board.place_piece(ChessPiece::new(Color::White, PieceType::Pawn), i, 1);
            board.place_piece(ChessPiece::new(Color::Black, PieceType::Pawn), i, 6);
        }

        board.place_piece(ChessPiece::new(Color::White, PieceType::Rook), 0, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Rook), 7, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Rook), 0, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Rook), 7, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Knight), 1, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Knight), 6, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Knight), 1, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Knight), 6, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Bishop), 2, 0);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Bishop), 5, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Bishop), 2, 7);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Bishop), 5, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::Queen), 3, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::Queen), 3, 7);
        board.place_piece(ChessPiece::new(Color::White, PieceType::King), 4, 0);
        board.place_piece(ChessPiece::new(Color::Black, PieceType::King), 4, 7);

        board
    }
}
