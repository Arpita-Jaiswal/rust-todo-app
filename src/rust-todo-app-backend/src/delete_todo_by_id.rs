#[ic_cdk::update]
pub(crate) fn delete_todo_by_id(id: todo_app::TodoId) -> Option<todo_app::Todo> {
    todo_app::TODOS.with(|todos| todos.borrow_mut().remove(&id))
}
