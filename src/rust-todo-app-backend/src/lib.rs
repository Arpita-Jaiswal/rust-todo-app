extern crate self as todo_app;

mod add_todo;
mod get_todo_by_id;
mod get_all_todos;
mod update_todo;
mod delete_todo_by_id;


/// `TodoId` is defined as a `u64` (unsigned 64-bit integer) and is used throughout the application
/// to uniquely identify each todo item.
type TodoId = u64;



/// Represents a todo item in the application.
///
/// The `Todo` struct stores the unique identifier (`id`) and the textual description (`text`)
/// of a todo item.
///
/// # Fields
///
/// - `id`: A unique identifier (`TodoId`) for the todo item.
/// - `text`: A `String` that contains the description or content of the todo item.

#[derive(Debug, Clone, PartialEq, candid::CandidType, serde::Deserialize)]
struct Todo {
    id: TodoId,
    text: String,
}



// Thread-local storage for managing todos.
//
// The `TODOS` variable is a thread-local storage containing a `RefCell` wrapped `BTreeMap` that maps
// `TodoId` to `Todo` instances. The `BTreeMap` is chosen because it keeps the todos in a sorted order
// by their IDs, ensuring that todos are retrieved in the order they were added.
//
// The `NEXT_ID` variable keeps track of the next unique identifier to be assigned to a new todo item.
thread_local! {
    static TODOS: std::cell::RefCell<std::collections::BTreeMap<TodoId, Todo>> = std::cell::RefCell::new(std::collections::BTreeMap::new());
    static NEXT_ID: std::cell::RefCell<TodoId> = std::cell::RefCell::new(0);
}
