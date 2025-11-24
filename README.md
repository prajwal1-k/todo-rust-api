# 📝 Todo Application  
### Built with Rust (Actix-web) + HTML, CSS & JavaScript

A lightweight full-stack Todo application featuring a high-performance Rust backend and a simple HTML/CSS/JS frontend.  
This project demonstrates how to build and connect a REST API using Actix-web with a traditional static frontend.

---

## 🚀 Tech Stack

### **Backend — Rust (Actix-web)**
- Actix-web (fast, scalable web framework)
- Serde (JSON serialization)
- UUID (unique todo IDs)
- `Mutex + HashMap` (in-memory database)
- CORS enabled for frontend communication
- Modular file structure (models, handlers, routes)

### **Frontend — HTML, CSS, JavaScript**
- Simple & clean UI
- JavaScript Fetch API to call backend
- No frameworks required
- Runs on any static server

---

## 📦 Features

### ✅ Backend (REST API)
- Create a Todo  
- Get all Todos  
- Update an existing Todo  
- Delete a Todo  
- JSON request/response support  
- Clean separation of logic  

### 🎨 Frontend
- Add new todos  
- Display todos  
- Toggle completion  
- Delete todos  
- Fully synced with the backend API  

---

## 📂 Folder Structure

```md
todo-app/
│
├── backend/
│   ├── src/
│   │   ├── handlers.rs
│   │   ├── models.rs
│   │   ├── routes.rs
│   │   └── main.rs
│   └── Cargo.toml
│
└── frontend/
    ├── index.html
    ├── style.css
    └── app.js
```
---

## 🏃‍♂️ How to Run the Project

This project contains two parts:  
- **Backend** (Rust + Actix-web)  
- **Frontend** (HTML, CSS, JS)

Follow the steps below to run both correctly.

---

## 🔧 1. Run the Backend (Rust + Actix-web)

### **Step 1 — Go to the backend folder**
```sh
cd todo-app/backend
```

### **Step 2- Install Dependencies**
```sh
cargo build
```

### **Step 3 - Run the Server**
```sh
cargo run
```

### **Step 4 - ✔️ Backend will start at:**
```sh
http://127.0.0.1:8080
```

---
## 🤝 Contribution

Contributions are welcome!

---

## 📚 Resources

Here are useful resources used in this project:

- **Actix-web Documentation**  
  https://actix.rs/

- **The Rust Programming Language (Rust Book)**  
  https://doc.rust-lang.org/book/

- **Serde JSON**  
  https://serde.rs/
- **Stack Overflow Disscussions**

