const API_URL = "http://127.0.0.1:8080/todos";

// Load todos when site opens
window.onload = loadTodos;

// Fetch All Todos
async function loadTodos() {
    const res = await fetch(API_URL);
    const todos = await res.json();

    const list = document.getElementById("todoList");
    list.innerHTML = "";

    todos.forEach(todo => {
        let li = document.createElement("li");

        li.innerHTML = `
            <span class="${todo.completed ? "complete" : ""}">
                ${todo.title}
            </span>

            <div>
                <button onclick="toggleComplete('${todo.id}', ${!todo.completed})">
                    ✔
                </button>
                <button onclick="deleteTodo('${todo.id}')">
                    ❌
                </button>
            </div>
        `;

        list.appendChild(li);
    });
}

// Create Todo
async function createTodo() {
    const input = document.getElementById("todoInput");

    if (input.value.trim() === "") return;

    await fetch(API_URL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title: input.value })
    });

    input.value = "";
    loadTodos();
}

// Toggle Complete / Update Todo
async function toggleComplete(id, completed) {
    await fetch(`${API_URL}/${id}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ completed })
    });

    loadTodos();
}

// Delete Todo
async function deleteTodo(id) {
    await fetch(`${API_URL}/${id}`, {
        method: "DELETE"
    });

    loadTodos();
}
