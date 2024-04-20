/**
 * @typedef PaginatedView
 * @template T
 * @type {object}
 * @property {T[]} items
 * @property {number} page
 * @property {number} size
 */

/**
 * @typedef Todo
 * @type {object}
 * @property {string} id
 * @property {string} title
 * @property {string} description
 * @property {boolean} completed
 * @property {string} createdAt
 * @property {string} updatedAt
 */

/**
 * Fetch all todos
 *
 * @param {object} todoRequest
 * @param {string} todoRequest.title
 * @param {string} todoRequest.description
 * @returns {Promise<Todo>} The created todo
 */
function createTodo(todoRequest) {
  const { title, description } = todoRequest;

  return fetch("/api/todos", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${getAuth()}`,
    },
    body: JSON.stringify({ title, description }),
  }).then((response) => response.json());
}

/**
 * Fetch all todos
 *
 * @param {object} [findTodosRequest]
 * @param {number} findTodosRequest.page
 * @param {number} findTodosRequest.size
 * @returns {Promise<PaginatedView<Todo>>} The paginated todos
 */
function findTodos(findTodosRequest) {
  const { page = 1, size = 10 } = findTodosRequest || {};

  return fetch("/api/todos?page=" + page + "&size=" + size, {
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${getAuth()}`,
    },
  }).then((response) => response.json());
}

/**
 * Edit todo by id
 *
 * @param {number} todoId
 * @param {object} editTodoRequest
 * @param {string} editTodoRequest.title
 * @param {string} editTodoRequest.description
 * @param {boolean} editTodoRequest.completed
 * @returns {Promise<Todo>} The updated todo
 */
function editTodo(todoId, editTodoRequest) {
  return fetch(`/api/todos/${todoId}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${getAuth()}`,
    },
    body: JSON.stringify({ ...editTodoRequest }),
  }).then((response) => response.json());
}

/**
 * Delete todo by id
 *
 * @param {number} todoId
 * @returns {Promise<void>} The deleted todo
 */
function deleteTodo(todoId) {
  return fetch(`/api/todos/${todoId}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${getAuth()}`,
    },
  });
}
