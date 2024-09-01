/// Deletes a todo item from the collection by its ID.
///
/// This function removes a todo item from the thread-local storage based on the provided ID.
/// If a todo with the given ID exists, it is removed and returned. If no todo with the
/// given ID is found, `None` is returned.
///
/// # Arguments
///
/// * `id` - The `TodoId` of the todo item to be deleted.
///
/// # Returns
///
/// * `Option<todo_app::Todo>` - Returns `Some(Todo)` if the todo was found and deleted, or
///   `None` if no todo with the given ID exists.
///
/// # Example
///
/// ```
/// let deleted_todo = delete_todo_by_id(1);
/// match deleted_todo {
///     Some(todo) => println!("Deleted todo: {}", todo.text),
///     None => println!("Todo with ID 1 not found."),
/// }
/// ```
#[ic_cdk::update]
pub(crate) fn delete_todo_by_id(id: todo_app::TodoId) -> Option<todo_app::Todo> {
    todo_app::TODOS.with(|todos| todos.borrow_mut().remove(&id))
}



#[cfg(test)]
mod tests {
    use super::delete_todo_by_id;
    use todo_app::add_todo::add_todo;
    use todo_app::get_todo_by_id::get_todo_by_id;

    #[test]
    fn test_delete_todo_by_id() {
        let id = add_todo("Test Todo".to_string());
        let todo = get_todo_by_id(id).unwrap();
        let deleted_todo = delete_todo_by_id(id);
        assert_eq!(deleted_todo, Some(todo));
    }

    #[test]
    fn test_delete_nonexistent_todo_by_id() {
        let deleted_todo = delete_todo_by_id(1);
        assert_eq!(deleted_todo, None);
    }
}