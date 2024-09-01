#[ic_cdk::update]
pub(crate) fn update_todo(id: todo_app::TodoId, text: String) -> Option<todo_app::Todo> {
    todo_app::TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if let Some(todo) = todos.get_mut(&id) {
            todo.text = text.clone();
            return Some(todo.clone());
        }
        None
    })
}
