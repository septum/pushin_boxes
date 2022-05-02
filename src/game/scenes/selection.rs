use crate::state::Selection;

pub fn is_stock_selection(selection_kind: &Selection) -> bool {
    matches!(selection_kind, Selection::Stock)
}
