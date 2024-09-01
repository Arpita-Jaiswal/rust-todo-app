#[ic_cdk::update]
pub(crate) fn add_todo(text: String) -> todo_app::TodoId {
    let id = todo_app::NEXT_ID.with(|id| {
        let new_id = *id.borrow();
        *id.borrow_mut() += 1;
        new_id
    });

    let todo = todo_app::Todo { id, text };

    todo_app::TODOS.with(|todos| {
        todos.borrow_mut().insert(id, todo.clone());
    });

    id
}
