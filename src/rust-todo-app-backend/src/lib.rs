extern crate self as todo_app;

mod add_todo;
mod delete_todo_by_id;
mod get_all_todos;
mod get_todo_by_id;
mod update_todo;

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

impl ic_stable_structures::Storable for Todo {
    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        use candid::Encode;

        std::borrow::Cow::Owned(Encode!(self).expect("Failed to serialize Todo"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        use candid::Decode;

        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

type Memory = ic_stable_structures::RestrictedMemory<ic_stable_structures::DefaultMemoryImpl>;

// Thread-local storage for managing todos.
//
// The `TODOS` variable is a thread-local storage containing a `RefCell` wrapped `BTreeMap` that maps
// `TodoId` to `Todo` instances. The `BTreeMap` is chosen because it keeps the todos in a sorted order
// by their IDs, ensuring that todos are retrieved in the order they were added.
//
// The `NEXT_ID` variable keeps track of the next unique identifier to be assigned to a new todo item.
thread_local! {
    // Initialize a `StableBTreeMap`. We're providing the map a `RestrictedMemory`,
    // which allows us to divide the stable memory into non-intersecting ranges
    // so that we can store multiple stable structures if we later wish.
    //
    // In this case, this map is given the range [0, 99], so it has access to the first
    // 100 pages in stable memory. Note that a page is 64KiB.
    //
    // Note that we can safely increase the range at any time (e.g. from 0..99 to 0..999)
    // to give the map more space to grow.
    static TODOS: std::cell::RefCell<ic_stable_structures::StableBTreeMap<TodoId, Todo, Memory>> =
        std::cell::RefCell::new(
            ic_stable_structures::StableBTreeMap::init(
                ic_stable_structures::RestrictedMemory::new(
                    ic_stable_structures::DefaultMemoryImpl::default(),
                    0..99
                )
            )
        );
    static NEXT_ID: std::cell::RefCell<ic_stable_structures::StableCell<TodoId, Memory>> =
        std::cell::RefCell::new(
            ic_stable_structures::StableCell::init(
                ic_stable_structures::RestrictedMemory::new(
                    ic_stable_structures::DefaultMemoryImpl::default(),
                    0..99
                ),
                0
            ).unwrap()
        );
}
