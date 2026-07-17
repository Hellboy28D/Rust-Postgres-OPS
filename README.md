# 🦀 Rust-Postgres-OPS

> A comprehensive Rust project demonstrating how to integrate PostgreSQL with Rust using the `postgres` crate. This repository covers database connectivity, CRUD operations, parameterized SQL queries, and best practices for building fast, safe, and scalable database applications in Rust.

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-blue?style=for-the-badge&logo=postgresql)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Active-success?style=for-the-badge)

---

# 📖 Table of Contents

- [About](#about)
- [Features](#features)
- [Project Structure](#project-structure)
- [Technology Stack](#technology-stack)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Database Setup](#database-setup)
- [Running the Project](#running-the-project)
- [Project Workflow](#project-workflow)
- [Example Output](#example-output)
- [CRUD Operations](#crud-operations)
- [Future Improvements](#future-improvements)
- [Learning Outcomes](#learning-outcomes)
- [Contributing](#contributing)
- [License](#license)

---

# About

Rust-Postgres-OPS is an educational project created to demonstrate how to build applications in **Rust** that communicate with a **PostgreSQL** database.

The project showcases fundamental database operations while emphasizing Rust's strengths:

- Memory safety
- High performance
- Type safety
- Error handling
- Zero-cost abstractions

Instead of relying on an ORM, this project uses the official **postgres** crate, allowing developers to understand exactly how SQL queries interact with Rust.

This repository serves as a practical starting point for developers interested in backend development with Rust.

---

# Features

✅ PostgreSQL Database Connection

✅ Create Database Tables

✅ Insert Records

✅ Read Records

✅ Parameterized SQL Queries

✅ Rust Struct Mapping

✅ Error Handling using `Result`

✅ Prepared Statements

✅ Secure SQL Execution

✅ Clean and Modular Code

---

# Project Structure

```
Rust-Postgres-OPS/
│
├── src/
│   └── main.rs
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
└── .gitignore
```

---

# Technology Stack

| Technology | Purpose |
|------------|---------|
| Rust | Programming Language |
| PostgreSQL | Relational Database |
| postgres crate | PostgreSQL Client |
| Cargo | Dependency Management |
| Git | Version Control |

---

# Prerequisites

Install the following before running the project.

## Rust

Install Rust:

https://rustup.rs

Verify installation:

```bash
rustc --version
cargo --version
```

---

## PostgreSQL

Download PostgreSQL:

https://www.postgresql.org/download/

Verify installation:

```bash
psql --version
```

---

# Installation

Clone the repository.

```bash
git clone https://github.com/Hellboy28D/Rust-Postgres-OPS.git
```

Move into the project.

```bash
cd Rust-Postgres-OPS
```

Install dependencies.

```bash
cargo build
```

---

# Database Setup

Create a PostgreSQL database.

```sql
CREATE DATABASE library;
```

Connect to the database.

```sql
\c library
```

The application automatically creates the required table if it does not exist.

Table Schema:

```sql
CREATE TABLE author(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    country VARCHAR(255)
);
```

---

# Running the Project

Run the application using Cargo.

```bash
cargo run
```

Cargo automatically compiles the project and executes it.

---

# Project Workflow

```
Rust Application

       │

       ▼

Connect to PostgreSQL

       │

       ▼

Create Table (if missing)

       │

       ▼

Insert Author Records

       │

       ▼

Execute SELECT Query

       │

       ▼

Map SQL Rows to Rust Structs

       │

       ▼

Display Results
```

---

# Example Output

```
Authors in the database

ID: 1
Name: Chinua Achebe
Country: Nigeria

ID: 2
Name: Rabindranath Tagore
Country: India

ID: 3
Name: Anita Nair
Country: India
```

---

# CRUD Operations

## Create

```rust
client.execute(
    "INSERT INTO author(name, country) VALUES ($1, $2)",
    &[&author.name, &author.country],
)?;
```

---

## Read

```rust
client.query(
    "SELECT id, name, country FROM author",
    &[],
)?;
```

---

## Update

```rust
client.execute(
    "UPDATE author SET country=$1 WHERE id=$2",
    &[&country, &id],
)?;
```

---

## Delete

```rust
client.execute(
    "DELETE FROM author WHERE id=$1",
    &[&id],
)?;
```

---

# Error Handling

Rust encourages robust error handling.

```rust
fn main() -> Result<(), Error>
```

The `?` operator propagates errors automatically, making the code concise and safe.

---

# Why Rust?

Rust offers several advantages for backend development:

- Fast execution
- Zero-cost abstractions
- Strong compile-time guarantees
- Thread safety
- Excellent concurrency support
- Memory safety without garbage collection

These features make Rust an excellent choice for high-performance database applications.

---

# Why PostgreSQL?

PostgreSQL is one of the world's most advanced open-source relational databases.

Benefits include:

- ACID Compliance
- Transactions
- Indexing
- JSON Support
- Extensions
- Reliability
- Scalability

---

# Learning Outcomes

After completing this project, you will understand:

- Connecting Rust to PostgreSQL
- Writing SQL inside Rust
- Parameterized queries
- Mapping database rows to Rust structs
- Handling database errors
- Executing CRUD operations
- Organizing Rust backend projects

---

# Future Improvements

The following enhancements are planned:

- [ ] Async PostgreSQL using tokio-postgres
- [ ] Connection Pooling
- [ ] Diesel ORM Integration
- [ ] SQLx Integration
- [ ] REST API with Axum
- [ ] Authentication
- [ ] Docker Support
- [ ] Environment Variables
- [ ] Logging
- [ ] Unit Testing
- [ ] Integration Testing
- [ ] Pagination
- [ ] Transactions
- [ ] Repository Pattern
- [ ] CI/CD using GitHub Actions

---

# Contributing

Contributions are welcome.

1. Fork the repository

2. Create a new branch

```bash
git checkout -b feature-name
```

3. Commit changes

```bash
git commit -m "Added new feature"
```

4. Push the branch

```bash
git push origin feature-name
```

5. Open a Pull Request

---

# License

This project is licensed under the MIT License.

Feel free to use this project for learning, experimentation, and educational purposes.

---

# Author

**Hellboy28D**

GitHub: https://github.com/Hellboy28D

---

## ⭐ If you found this project helpful, consider giving it a Star!

A star helps support the project and motivates further development. Happy coding! 🦀
