use crate::board::BoardAccess;
use std::io::Write;

pub fn reindex<B: BoardAccess>(
    board: &mut B,
    _writer: &mut dyn Write,
){
    let _ = board.reindex_columns();
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test::StoreMock, board::Board};
    use std::{str, io::Cursor};

    fn it_calls_reindex_on_board() {
        let mut writer = Cursor::new(vec!());
        let mut store = StoreMock::new();
        let mut col_store = StoreMock::new();
        let mut board = Board::new(&mut store, &mut col_store);

        reindex(&mut board, &mut writer);

        let output = writer.get_ref();
    }
}
*/
