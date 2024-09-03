/// Updates the text of an existing todo item by its ID.
///
/// This function updates the `text` field of a todo item identified by the given `id`.
/// If a todo with the specified ID exists, its text is updated, and the updated todo is returned.
/// If no todo with the given ID is found, `None` is returned.
///
/// # Arguments
///
/// * `id` - The `TodoId` of the todo item to be updated.
/// * `text` - A `String` containing the new text or description for the todo item.
///
/// # Returns
///
/// * `Option<todo_app::Todo>` - Returns `Some(Todo)` with the updated text if the todo is found and updated,
///   otherwise returns `None` if no todo with the specified ID exists.
///
/// # Example
///
/// ```
/// let updated_todo = update_todo(1, "New updated text".to_string());
/// match updated_todo {
///     Some(todo) => println!("Updated todo: {}", todo.text),
///     None => println!("Todo with ID 1 not found."),
/// }
/// ```
#[ic_cdk::update]
pub(crate) fn update_todo(id: todo_app::TodoId, text: String) -> Option<todo_app::Todo> {
    todo_app::TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();

        // Return None if no todo with the given ID exists
        if !todos.contains_key(&id) {
            return None;
        }

        // Update the text of the todo with the given ID
        let todo = todo_app::Todo { id, text };
        todos.insert(id, todo.clone());
        Some(todo)
    })
}

#[cfg(test)]
mod tests {
    use super::update_todo;
    use todo_app::add_todo::add_todo;
    use todo_app::get_todo_by_id::get_todo_by_id;

    #[test]
    fn test_update_todo() {
        let id = add_todo("Test Todo".to_string());
        let todo = get_todo_by_id(id).unwrap();
        assert_eq!(todo.text, "Test Todo");

        let updated_todo = update_todo(id, "New updated text".to_string());
        let todo = get_todo_by_id(id).unwrap();
        assert_eq!(updated_todo, Some(todo));
    }

    #[test]
    fn test_update_nonexistent_todo() {
        let updated_todo = update_todo(1, "New updated text".to_string());
        assert_eq!(updated_todo, None);
    }
}
