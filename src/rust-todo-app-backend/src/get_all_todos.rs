#[ic_cdk::query]
pub(crate) fn get_all_todos(page: u64, limit: u64) -> Vec<todo_app::Todo> {
    todo_app::TODOS.with(|todos| {
        let todos = todos.borrow();
        todos
            .values()
            .skip((page * limit) as usize)
            .take(limit as usize)
            .cloned()
            .collect()
    })
}
