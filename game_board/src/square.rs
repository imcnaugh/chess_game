use crate::color::SquareColor;
use core::fmt;
use std::fmt::{Display, Formatter};

/// Converts a given column and row to a simple_chess-style coordinate string.
///
/// The function works by repeatedly dividing the column index by 26 to
/// convert it to a base-26 representation, with 'a' representing 1, 'b'
/// representing 2, and so on. This representation is reversed and
/// combined with the row (incremented by 1) to produce the final coordinate.
///
/// # Arguments
///
/// * `column` - A `usize` representing the column index, where 0 corresponds to 'a'.
/// * `row` - A `usize` representing the row index, which is zero-based.
///
/// # Returns
///
/// A `String` containing the coordinate in simple_chess notation.
///
/// # Examples
///
/// ```
/// use game_board::get_square_name_from_row_and_col;
/// let coordinate = get_square_name_from_row_and_col(0, 0);
/// assert_eq!(coordinate, "a1");
///
/// let coordinate = get_square_name_from_row_and_col(25, 1);
/// assert_eq!(coordinate, "z2");
///
/// let coordinate = get_square_name_from_row_and_col(26, 0);
/// assert_eq!(coordinate, "aa1");
/// ```
pub fn get_square_name_from_row_and_col(column: usize, row: usize) -> String {
    let mut col_id = String::new();
    let mut remainder = column;

    loop {
        let r = remainder % 26;
        let c = (r as u8 + b'a') as char;
        col_id.push(c);
        remainder /= 26;
        if remainder == 0 {
            break;
        }
        remainder -= 1;
    }

    col_id = col_id.chars().rev().collect();

    format!("{}{}", col_id, row + 1)
}

/// Converts a simple_chess-style coordinate string to a given column and row.
///
/// The function works by separating the alphabetic characters (columns)
/// from the numeric characters (rows) in the input string. Once split,
/// it converts the alphabetic column to a base-26 number and adjusts it
/// to be zero-based. It also converts the numeric row to be zero-based.
///
/// # Arguments
///
/// * `name` - A `&str` representing the coordinate in simple_chess notation,
///            with alphabetic characters for the column and numeric
///            characters for the row. Examples include "a1", "b2", "z2", etc.
///
/// # Returns
///
/// A `Result<(usize, usize), &str>` containing a tuple with the column
/// and row indices as `usize` if the input is valid, or an error string
/// if the input is invalid.
///
/// # Errors
///
/// This function returns an error if the input contains any invalid
/// characters (non-alphabetic characters in the column part or
/// non-numeric characters in the row part), or if the column or
/// row parts are empty.
///
/// # Examples
///
/// ```
/// use game_board::get_column_and_row_from_square_name;
///
/// let (column, row) = get_column_and_row_from_square_name("a1").unwrap();
/// assert_eq!(column, 0);
/// assert_eq!(row, 0);
///
/// let (column, row) = get_column_and_row_from_square_name("b2").unwrap();
/// assert_eq!(column, 1);
/// assert_eq!(row, 1);
///
/// let (column, row) = get_column_and_row_from_square_name("ab1").unwrap();
/// assert_eq!(column, 27);
/// assert_eq!(row, 0);
///
/// let (column, row) = get_column_and_row_from_square_name("zzz100").unwrap();
/// assert_eq!(column, 18277);
/// assert_eq!(row, 99);
/// ```
///
/// # Panics
///
/// The function will panic if it fails to parse the row part into a `usize`.
pub fn get_column_and_row_from_square_name(name: &str) -> Result<(usize, usize), &str> {
    let mut col_as_string = String::new();
    let mut row_as_string = String::new();
    let mut finding_col = true;

    for c in name.chars() {
        if finding_col {
            if c.is_ascii_alphabetic() {
                col_as_string.push(c);
            } else if c.is_ascii_digit() {
                finding_col = false;
                row_as_string.push(c);
            } else {
                return Err("Invalid input");
            }
        } else if c.is_ascii_digit() {
            row_as_string.push(c);
        } else {
            return Err("Invalid input");
        }
    }
    if col_as_string.is_empty() || row_as_string.is_empty() {
        return Err("Invalid input");
    }

    let mut column: usize = 0;
    for (index, c) in col_as_string.chars().enumerate() {
        let base_26 = c as usize - 'a' as usize + 1;
        let s = 26_usize.pow((col_as_string.len() - index - 1) as u32);

        column += s * base_26;
    }

    let row: usize = row_as_string.parse().unwrap();
    Ok((column - 1, row - 1))
}

/// Represents a square on a simple_chess board.
///
/// The `Square` struct holds the column and row indices, the color of the square,
/// and an optional piece that might occupy the square. The color is determined based
/// on the column and row indices.
///
/// # Type Parameters
///
/// * `P` - A type that implements the `Piece` trait, representing the type piece that can be placed on the square.
///
/// # Fields
///
/// * `column` - The zero-based column index of the square.
/// * `row` - The zero-based row index of the square.
/// * `color` - The color of the square, which can be either white or black.
/// * `piece` - An optional field that holds a piece of type `P` if present on the square.
pub struct Square<P> {
    column: usize,
    row: usize,
    color: SquareColor,
    piece: Option<P>,
}

impl<P> Square<P> {
    /// Constructs a new `Square` with the specified column and row indices.
    /// The color of the square is determined by the parity of the sum of
    /// the column and row indices.
    ///
    /// # Arguments
    ///
    /// * `column` - The zero-based column index of the square.
    /// * `row` - The zero-based row index of the square.
    ///
    /// # Returns
    ///
    /// A `Square` instance with the specified column and row, and the calculated color.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_board::{Square, SquareColor};
    /// let square = Square::<String>::build(0, 0);
    /// assert_eq!(square.get_color(), SquareColor::Black);
    /// let square = Square::<i32>::build(1, 0);
    /// assert_eq!(square.get_color(), SquareColor::White);
    /// ```
    ///
    pub fn build(column: usize, row: usize) -> Self {
        let color = if (column + row) % 2 == 1 {
            SquareColor::White
        } else {
            SquareColor::Black
        };
        Square {
            color,
            piece: None,
            column,
            row,
        }
    }

    /// Places a piece on the square.
    ///
    /// # Arguments
    ///
    /// * `piece` - A piece of type P
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    /// use game_board::Square;
    ///
    /// struct Pawn;
    ///
    /// let mut square = Square::build(0, 0);
    /// square.place_piece(Box::new(Pawn));
    /// assert!(square.get_piece().is_some());
    /// ```
    pub fn place_piece(&mut self, piece: P) {
        self.piece = Some(piece);
    }

    /// Returns a reference to the piece placed on the square, if any.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the piece of type `P`
    /// if a piece is placed on the square, or `None` if the square is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    /// use game_board::Square;
    ///
    /// struct Pawn;
    ///
    /// let mut square = Square::build(0, 0);
    /// square.place_piece(Pawn {});
    /// assert!(square.get_piece().is_some());
    /// ```
    pub fn get_piece(&self) -> Option<&P> {
        self.piece.as_ref()
    }

    /// Clears the piece from the square.
    ///
    /// # Returns
    ///
    /// An `Option` containing the piece of type `P` if a piece was present on the square,
    /// or `None` if the square was already empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    /// use game_board::Square;
    ///
    /// struct Pawn;
    ///
    /// let mut square = Square::build(0, 0);
    /// square.place_piece(Pawn {});
    /// let piece = square.clear_piece();
    /// assert!(piece.is_some());
    /// assert!(square.get_piece().is_none());
    /// ```
    pub fn clear_piece(&mut self) -> Option<P> {
        self.piece.take()
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_color(&self) -> SquareColor {
        self.color
    }

    /// Gets the name of the square in standard simple_chess notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_board::Square;
    ///
    /// let square = Square::<u32>::build(0, 0);
    /// assert_eq!(square.get_name(), "a1".to_string());
    ///
    /// let square = Square::<String>::build(25, 1);
    /// assert_eq!(square.get_name(), "z2".to_string());
    /// ```
    pub fn get_name(&self) -> String {
        get_square_name_from_row_and_col(self.column, self.row)
    }
}

impl<P: Display> Display for Square<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let square_color = match &self.color {
            SquareColor::White => "\x1b[100m",
            SquareColor::Black => "",
        };
        let inner_char = match &self.piece {
            Some(piece) => piece.to_string(),
            None => " ".to_string(),
        };
        write!(f, "{} {} \x1b[0m", square_color, inner_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPiece {}

    #[test]
    fn col_row_turn_into_id() {
        let square_a1 = get_square_name_from_row_and_col(0, 0);
        assert_eq!("a1", format!("{square_a1}"));

        let square_z2 = get_square_name_from_row_and_col(25, 1);
        assert_eq!("z2", format!("{square_z2}"));

        let square_aa1 = get_square_name_from_row_and_col(26, 0);
        assert_eq!("aa1", format!("{square_aa1}"));

        let square_ab1 = get_square_name_from_row_and_col(27, 0);
        assert_eq!("ab1", format!("{square_ab1}"));

        let square_zzz100 = get_square_name_from_row_and_col(18277, 99);
        assert_eq!("zzz100", format!("{square_zzz100}"));
    }

    #[test]
    fn string_to_id() {
        let (a1_column, a1_row) = get_column_and_row_from_square_name("a1").unwrap();
        assert_eq!(0, a1_column);
        assert_eq!(0, a1_row);

        let (b2_column, b2_row) = get_column_and_row_from_square_name("b2").unwrap();
        assert_eq!(1, b2_column);
        assert_eq!(1, b2_row);

        let (ab1_column, ab1_row) = get_column_and_row_from_square_name("ab1").unwrap();
        assert_eq!(27, ab1_column);
        assert_eq!(0, ab1_row);

        let (zzz100_column, zzz100_row) = get_column_and_row_from_square_name("zzz100").unwrap();
        assert_eq!(18277, zzz100_column);
        assert_eq!(99, zzz100_row);
    }

    #[test]
    fn test_square_build() {
        let square = Square::<MockPiece>::build(0, 0);
        assert_eq!(square.get_column(), 0);
        assert_eq!(square.get_row(), 0);
        assert_eq!(square.get_color(), SquareColor::Black); // a8
        assert!(square.get_piece().is_none());

        let square = Square::<MockPiece>::build(1, 0);
        assert_eq!(square.get_column(), 1);
        assert_eq!(square.get_row(), 0);
        assert_eq!(square.get_color(), SquareColor::White); // b8
    }

    #[test]
    fn test_place_piece() {
        struct Pawn;

        let mut square = Square::build(0, 0);
        square.place_piece(Pawn);
        assert!(square.get_piece().is_some());
    }

    #[test]
    fn test_get_piece() {
        struct Pawn;

        let mut square = Square::build(0, 0);
        square.place_piece(Pawn);
        let piece = square.get_piece();
        assert!(piece.is_some());
    }

    #[test]
    fn test_clear_piece() {
        struct Pawn;

        let mut square = Square::build(0, 0);
        square.place_piece(Pawn);
        let piece = square.clear_piece();
        assert!(piece.is_some());
        assert!(square.get_piece().is_none());
    }

    #[test]
    fn test_get_column() {
        let square = Square::<MockPiece>::build(5, 3);
        assert_eq!(square.get_column(), 5);
    }

    #[test]
    fn test_get_row() {
        let square = Square::<MockPiece>::build(5, 3);
        assert_eq!(square.get_row(), 3);
    }

    #[test]
    fn test_get_color() {
        let square = Square::<MockPiece>::build(0, 0);
        assert_eq!(square.get_color(), SquareColor::Black);
    }

    #[test]
    fn test_get_name() {
        let square = Square::<MockPiece>::build(0, 0);
        assert_eq!(square.get_name(), "a1".to_string());

        let square = Square::<MockPiece>::build(25, 1);
        assert_eq!(square.get_name(), "z2".to_string());
    }

    #[test]
    fn can_print_square() {
        struct Printable {}
        impl Display for Printable {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "Hello world")
            }
        }

        let square = Square::<Printable>::build(0, 0);
        println!("{}", square);
    }
}
