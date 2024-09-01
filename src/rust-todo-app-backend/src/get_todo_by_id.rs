#[ic_cdk::query]
pub(crate) fn get_todo_by_id(id: todo_app::TodoId) -> Option<todo_app::Todo> {
    todo_app::TODOS.with(|todos| todos.borrow().get(&id).cloned())
}
