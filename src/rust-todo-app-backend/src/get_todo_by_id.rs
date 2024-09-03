/// Retrieves a todo item by its ID.
///
/// This function looks up a todo item in the collection based on the provided ID.
/// If a todo with the given ID exists, it returns `Some(Todo)` containing the todo details.
/// If no todo with the given ID is found, it returns `None`.
///
/// # Arguments
///
/// * `id` - The `TodoId` of the todo item to be retrieved.
///
/// # Returns
///
/// * `Option<todo_app::Todo>` - Returns `Some(Todo)` if a todo with the specified ID is found,
///   otherwise returns `None`.
///
/// # Example
///
/// ```
/// let todo = get_todo_by_id(1);
/// match todo {
///     Some(todo) => println!("Found todo: {}", todo.text),
///     None => println!("Todo with ID 1 not found."),
/// }
/// ```
#[ic_cdk::query]
pub(crate) fn get_todo_by_id(id: todo_app::TodoId) -> Option<todo_app::Todo> {
    todo_app::TODOS.with(|todos| {
        todos.borrow().get(&id)
    })
}

#[cfg(test)]
mod tests {
    use super::get_todo_by_id;
    use todo_app::add_todo::add_todo;

    #[test]
    fn test_get_todo_by_id() {
        let id = add_todo("Test Todo".to_string());
        let todo = get_todo_by_id(id).unwrap();
        assert_eq!(todo.text, "Test Todo");
    }

    #[test]
    fn test_get_nonexistent_todo_by_id() {
        let todo = get_todo_by_id(1);
        assert_eq!(todo, None);
    }
}
