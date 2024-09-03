/// Retrieves a paginated list of todo items.
///
/// This function fetches todo items from the collection, returning a specified number of items (`limit`)
/// starting from a specified page (`page`). Pagination is zero-based, meaning `page = 0` returns the first
/// set of todos. The function returns a vector containing the requested todos.
///
/// # Arguments
///
/// * `page` - A `u64` representing the page number to retrieve. Each page contains up to `limit` items.
/// * `limit` - A `u64` representing the maximum number of todos to return in one page.
///
/// # Returns
///
/// * `Vec<todo_app::Todo>` - A vector containing the todos on the specified page, up to the specified limit.
///   If the requested page is out of bounds, an empty vector is returned.
///
/// # Example
///
/// ```
/// let todos_page_1 = get_all_todos(0, 10);  // Retrieves the first 10 todos.
/// for todo in todos_page_1 {
///     println!("Todo ID: {}, Text: {}", todo.id, todo.text);
/// }
/// ```
#[ic_cdk::query]
pub(crate) fn get_all_todos(page: u64, limit: u64) -> Vec<todo_app::Todo> {
    todo_app::TODOS.with(|todos| {
        let todos = todos.borrow();

        todos
            .iter()
            .map(|(_, todo)| todo)
            .skip((page * limit) as usize)
            .take(limit as usize)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::get_all_todos;
    use todo_app::add_todo::add_todo;

    #[test]
    fn test_get_all_todos() {
        let _id = add_todo("Test TODO".to_string());
        let todos = get_all_todos(0, 10);
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Test TODO");
    }

    #[test]
    fn test_get_all_todos_with_offset() {
        let _id = add_todo("Test TODO 1".to_string());
        let todos = get_all_todos(0, 10);
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Test TODO 1");

        let _id = add_todo("Test TODO 2".to_string());
        let todos = get_all_todos(0, 10);
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Test TODO 1");
        assert_eq!(todos[1].text, "Test TODO 2");

        let todos = get_all_todos(1, 1);
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Test TODO 2");

        let todos = get_all_todos(5, 1);
        assert_eq!(todos.len(), 0);
    }

    #[test]
    fn test_get_all_todos_with_offset_and_limit() {
        let _id = add_todo("Test TODO 1".to_string());
        let _id = add_todo("Test TODO 2".to_string());
        let _id = add_todo("Test TODO 3".to_string());
        let _id = add_todo("Test TODO 4".to_string());
        let _id = add_todo("Test TODO 5".to_string());
        let _id = add_todo("Test TODO 6".to_string());
        let _id = add_todo("Test TODO 7".to_string());
        let _id = add_todo("Test TODO 8".to_string());
        let _id = add_todo("Test TODO 9".to_string());
        let _id = add_todo("Test TODO 10".to_string());

        let todos = get_all_todos(0, 2);
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Test TODO 1");
        assert_eq!(todos[1].text, "Test TODO 2");

        let todos = get_all_todos(1, 2);
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Test TODO 3");
        assert_eq!(todos[1].text, "Test TODO 4");

        let todos = get_all_todos(2, 2);
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Test TODO 5");
        assert_eq!(todos[1].text, "Test TODO 6");

        let todos = get_all_todos(3, 2);
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Test TODO 7");
        assert_eq!(todos[1].text, "Test TODO 8");

        let todos = get_all_todos(4, 2);
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Test TODO 9");
        assert_eq!(todos[1].text, "Test TODO 10");

        let todos = get_all_todos(0, 5);
        assert_eq!(todos.len(), 5);
        assert_eq!(todos[0].text, "Test TODO 1");
        assert_eq!(todos[1].text, "Test TODO 2");
        assert_eq!(todos[2].text, "Test TODO 3");
        assert_eq!(todos[3].text, "Test TODO 4");
        assert_eq!(todos[4].text, "Test TODO 5");

        let todos = get_all_todos(1, 5);
        assert_eq!(todos.len(), 5);
        assert_eq!(todos[0].text, "Test TODO 6");
        assert_eq!(todos[1].text, "Test TODO 7");
        assert_eq!(todos[2].text, "Test TODO 8");
        assert_eq!(todos[3].text, "Test TODO 9");
        assert_eq!(todos[4].text, "Test TODO 10");
    }
}
