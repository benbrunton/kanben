use crate::opts::Column;
use crate::board::BoardAccess;

pub fn add_item<B: BoardAccess>(name: String, board: &mut B){
    if is_valid_key(&name) {
        board.create_task(&name, None);
    }
}

fn is_valid_key(name: &str) -> bool {
    name.trim().len() > 0
}

pub fn start_item<B: BoardAccess>(name: String, board: &mut B) {
    move_item(name, board, Column::Doing);
}

pub fn complete_item<B: BoardAccess>(name: String, board: &mut B) {
    move_item(name, board, Column::Done);
}

fn move_item<B: BoardAccess>(name: String, board: &mut B, column: Column) {
    let mut item = board.get(&name).unwrap();
    item.column = column;
    board.update(&name, item);
}

pub fn delete_item<B: BoardAccess>(name: String, board: &mut B) {
    board.remove(&name);
}

pub fn clear_done<B: BoardAccess>(board: &mut B) {
    board.get_all_tasks().iter().filter(|task| {
        task.column == Column::Done
    }).for_each(|task| board.remove(&task.name));
}

pub fn top<B: BoardAccess>(name: String, board: &mut B) {
    board.top_priority(&name);
}
