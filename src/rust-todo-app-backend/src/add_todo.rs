/// Adds a new todo item to the collection.
///
/// This function generates a new unique ID for the todo, creates a `Todo` struct with
/// the provided text, and stores it in the collection. The ID of the newly created todo
/// is then returned.
///
/// # Arguments
///
/// * `text` - A `String` containing the text or description of the todo item.
///
/// # Returns
///
/// * `todo_app::TodoId` - The ID of the newly created todo item.
///
/// # Example
///
/// ```
/// let id = add_todo("Finish writing Rust documentation".to_string());
/// println!("Created new todo with ID: {}", id);
/// ```
#[ic_cdk::update]
pub(crate) fn add_todo(text: String) -> todo_app::TodoId {
    let id = next_id();
    let todo = todo_app::Todo { id, text };
    insert_todo(id, todo);
    id
}

/// Generates the next unique ID for a todo item.
///
/// This function increments a thread-local counter to ensure each todo item has a unique ID.
///
/// # Returns
///
/// * `todo_app::TodoId` - The next unique ID to be used for a new todo item.
///
/// # Example
///
/// ```
/// let id = next_id();
/// println!("Next available ID: {}", id);
/// ```
fn next_id() -> todo_app::TodoId {
    todo_app::NEXT_ID.with(|id| {
        let mut id = id.borrow_mut();
        let new = *id.get() + 1;
        id.set(new).unwrap()
    })
}

/// Inserts a todo item into the collection.
///
/// This function stores a given `Todo` item in the thread-local storage, mapping it to
/// the provided ID. If an item with the same ID already exists, it will be overwritten.
///
/// # Arguments
///
/// * `id` - The `TodoId` associated with the todo item.
/// * `todo` - The `Todo` struct that contains the ID and text of the todo item.
///
/// # Example
///
/// ```
/// let todo = todo_app::Todo { id: 1, text: "Complete the report".to_string() };
/// insert_todo(1, todo);
/// ```
fn insert_todo(id: todo_app::TodoId, todo: todo_app::Todo) {
    todo_app::TODOS.with(|todos| {
        todos.borrow_mut().insert(id, todo.clone());
    })
}

#[cfg(test)]
mod tests {
    use super::add_todo;

    #[test]
    fn test_add_todo() {
        let id = add_todo("Test TODO".to_string());
        assert_eq!(id, 0);
    }
}
