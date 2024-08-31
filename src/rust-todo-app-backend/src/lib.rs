#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

use std::cell::RefCell;
use std::collections::HashMap;

type TodoId = u64;

#[derive(Debug, Clone, candid::CandidType, serde::Deserialize)]
struct Todo {
    id: TodoId,
    text: String,
}

thread_local! {
    static TODOS: RefCell<HashMap<TodoId, Todo>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<TodoId> = RefCell::new(0);
}

#[ic_cdk::update]
fn add_todo(text: String) -> TodoId {
    let id = NEXT_ID.with(|id| {
        let new_id = *id.borrow();
        *id.borrow_mut() += 1;
        new_id
    });

    let todo = Todo { id, text };

    TODOS.with(|todos| {
        todos.borrow_mut().insert(id, todo.clone());
        ic_cdk::println!("Added todo: {:?}", todo);
    });

    id
}

#[ic_cdk::query]
fn get_todo_by_id(id: TodoId) -> Option<Todo> {
    ic_cdk::println!("Added todo id: {id}");
    TODOS.with(|todos| todos.borrow().get(&id).cloned())
}

#[ic_cdk::query]
fn get_all_todos(page: u64, limit: u64) -> Vec<Todo> {
    TODOS.with(|todos| {
        let todos = todos.borrow();
        todos
            .values()
            .skip((page * limit) as usize)
            .take(limit as usize)
            .cloned()
            .collect()
    })
}

#[ic_cdk::update]
fn update_todo(id: TodoId, text: String) -> Option<Todo> {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if let Some(todo) = todos.get_mut(&id) {
            todo.text = text.clone();
            return Some(todo.clone());
        }
        None
    })
}

#[ic_cdk::update]
fn delete_todo_by_id(id: TodoId) -> Option<Todo> {
    TODOS.with(|todos| todos.borrow_mut().remove(&id))
}

// #[export_name = "canister_query __get_candid_interface"]
// fn export_candid() -> &'static str {
//     include_str!("../rust-todo-app-backend.did")
// }

#[cfg(test)]
mod tests {
    use super::*;

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
}
