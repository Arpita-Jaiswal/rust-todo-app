extern crate self as todo_app;

mod add_todo;
mod get_todo_by_id;
mod get_all_todos;
mod update_todo;
mod delete_todo_by_id;


type TodoId = u64;

#[derive(Debug, Clone, candid::CandidType, serde::Deserialize)]
struct Todo {
    id: TodoId,
    text: String,
}

thread_local! {
    static TODOS: std::cell::RefCell<std::collections::HashMap<TodoId, Todo>> = std::cell::RefCell::new(std::collections::HashMap::new());
    static NEXT_ID: std::cell::RefCell<TodoId> = std::cell::RefCell::new(0);
}



#[cfg(test)]
mod tests {
    use super::add_todo::add_todo;
    use super::get_todo_by_id::get_todo_by_id;
    use super::get_all_todos::get_all_todos;
    use super::update_todo::update_todo;
    use super::delete_todo_by_id::delete_todo_by_id;

    #[test]
    fn test_add_and_get_todo() {
        let id = add_todo("Test TODO".to_string());
        let todo = get_todo_by_id(id).unwrap();
        assert_eq!(todo.text, "Test TODO");
    }

    #[test]
    fn test_update_todo() {
        let id = add_todo("Old Text".to_string());
        update_todo(id, "New Text".to_string());
        let todo = get_todo_by_id(id).unwrap();
        assert_eq!(todo.text, "New Text");
    }

    #[test]
    fn test_delete_todo() {
        let id = add_todo("Delete Me".to_string());
        delete_todo_by_id(id);
        assert!(get_todo_by_id(id).is_none());
    }

    #[test]
    fn test_get_all_todos() {
        let _id = add_todo("Test TODO".to_string());
        let todos = get_all_todos(0, 10);
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Test TODO");
    }
}
