use super::*;

pub(super) fn row_count(object: &Object) -> Func {
    let mut func = Func::new(&format!("{}_row_count", snake_case(&object.name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .ret("c_int")
        .line("to_c_int((&*ptr).row_count())");

    func
}

pub(super) fn insert_rows(object: &Object) -> Func {
    let mut func = Func::new(&format!("{}_insert_rows", snake_case(&object.name)));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("row", "c_int")
        .arg("count", "c_int")
        .ret("bool");

    let mut match_block = Block::new("match (to_usize(row), to_usize(count))");
    match_block.line("(Some(row), Some(count)) => ");

    let mut insert_block = Block::new("");
    insert_block.line("(&mut *ptr).insert_rows(row, count)");

    match_block.push_block(insert_block);
    match_block.line("_ => false");

    func.push_block(match_block);

    func
}

pub(super) fn remove_rows(object: &Object) -> Func {
    let mut func = Func::new(&format!("{}_remove_rows", snake_case(&object.name)));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("row", "c_int")
        .arg("count", "c_int")
        .ret("bool");

    let mut match_block = Block::new("match (to_usize(row), to_usize(count))");
    match_block.line("(Some(row), Some(count)) => ");

    let mut remove_block = Block::new("");
    remove_block.line("(&mut *ptr).remove_rows(row, count)");

    match_block.push_block(remove_block);
    match_block.line("_ => false");

    func.push_block(match_block);

    func
}

pub(super) fn can_fetch_more(object: &Object) -> Func {
    let mut func = Func::new(&format!("{}_can_fetch_more", snake_case(&object.name)));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .ret("bool")
        .line("(&*ptr).can_fetch_more()");

    func
}

pub(super) fn fetch_more(object: &Object) -> Func {
    let mut func = Func::new(&format!("{}_fetch_more", snake_case(&object.name)));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .line("(&mut *ptr).fetch_more()");

    func
}

pub(super) fn sort(object: &Object) -> Func {
    let mut func = Func::new(&format!("{}_sort", snake_case(&object.name)));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("column", "u8")
        .arg("order", "SortOrder")
        .line("(&mut *ptr).sort(column, order)");

    func
}
