# Bug Tracker System - CSC1106 Practical Quiz
# Group 14 (g14_practical)

## Overview
A Rust-based web application for internal bug tracking system using Actix Web framework and SQLite database.
Built with enhanced security using bcrypt password hashing and JWT authentication.

## Features Implemented

### Core Requirements ✅
1. Bug Report Creation (POST /bugs/new)
   - Accepts JSON with title, description, reported_by, severity
   - Stores in SQLite database with auto-generated ID
   - Returns created bug record with assigned bug_id as JSON
   - Comprehensive validation and error handling

2. Project List State Management (GET/POST /projects)
   - Thread-safe in-memory project storage using Arc<RwLock<Vec<Project>>>
   - GET /projects - returns project list as JSON
   - POST /projects - adds new project (admin use only)
   - Pre-loaded with sample projects

3. User Login with Password Hashing (POST /login)
   - bcrypt hashing with fixed salt "bugtrack2025" as specified
   - Password hash = Hash(SALT + password) using bcrypt
   - Returns success/failure status with JWT session token
   - Pre-created admin user: admin/admin123

4. Bug Assignment with HTML Templates (GET/POST /bugs/assign)
   - Dynamic HTML generation with bug and developer listings
   - Bug assignment form at GET /bugs/assign
   - Form submission updates bug record with developer_id
   - Confirmation page with error handling for invalid IDs

5. Full CRUD for Bugs ✅
   - POST /bugs/new - Create new bug report
   - GET /bugs - List all bugs as JSON
   - GET /bugs/:id - Retrieve specific bug (404 if not found)
   - PATCH /bugs/:id - Update bug details
   - DELETE /bugs/:id - Delete bug (404 if missing)
   - Proper validation, error handling, and HTTP status codes

### 🚀 BONUS FEATURES IMPLEMENTED

6. Professional Homepage Dashboard
   - Interactive homepage at GET / with system overview
   - API documentation with all endpoints listed
   - Quick navigation links to main features
   - Copy-paste ready test commands
   - Modern responsive web design

7. JWT Token-Based Authentication System
   - Industry-standard JSON Web Tokens for session management
   - Token verification middleware for protected endpoints
   - GET /protected - demonstrates secure API access
   - Token expiration handling (24-hour validity)
   - Secure Authorization header parsing (Bearer token format)

8. Developer Management System
   - GET /developers - List all developers with full details
   - POST /developers - Add new developer to system
   - Auto-populated sample developers (John, Jane, Bob)
   - Integration with bug assignment workflow

9. Enhanced Security Features
   - bcrypt password hashing (industry standard vs basic SHA)
   - Salted password storage with fixed salt as required
   - SQL injection prevention via parameterized queries
   - Input validation and sanitization on all endpoints
   - Structured error responses without information leakage

10. Dynamic HTML Form Generation
    - Real-time bug and developer data in assignment forms
    - Dropdown selections populated from database
    - User-friendly interface with styling and validation
    - Success/failure confirmation pages with navigation

11. Professional API Architecture
    - Structured JSON responses with success/error status
    - Consistent response format across all endpoints
    - Comprehensive error handling with appropriate HTTP codes
    - RESTful API design principles

12. Database Schema Enhancements
    - Users table with role-based access control
    - Proper foreign key relationships (bugs -> developers)
    - Data integrity constraints and validation
    - Automatic sample data population on startup

13. Health Monitoring & Debugging
    - GET /health - System status endpoint for monitoring
    - Service information and version details
    - Admin password reset functionality (POST /fix-admin)
    - Comprehensive logging throughout application

14. Advanced State Management
    - Thread-safe concurrent access using Arc<RwLock>
    - In-memory project storage as specified
    - Atomic operations for data consistency
    - Scalable architecture for multiple users

15. Production-Ready Error Handling
    - Graceful error responses for all failure scenarios
    - Input validation with helpful error messages
    - Database error handling and recovery
    - Proper HTTP status codes (200, 201, 400, 404, 500)

## Technology Stack
- Framework: Actix Web 4.0 (async/await support)
- Database: SQLite with SQLx (compile-time verified queries)
- Authentication: bcrypt + JWT (industry standard security)
- Serialization: Serde JSON (type-safe serialization)
- Async Runtime: Tokio (high-performance async)
- Environment: dotenv for configuration management

## Database Schema

### Bugs Table (Existing Schema - Maintained)
- id (INTEGER PRIMARY KEY AUTOINCREMENT)
- title (TEXT NOT NULL)
- description (TEXT)
- reported_by (TEXT)
- severity (TEXT)
- developer_id (INTEGER - references developers.id)

### Developers Table (Existing Schema - Maintained)
- id (INTEGER PRIMARY KEY AUTOINCREMENT)
- name (TEXT NOT NULL)

### Users Table (Added for Authentication)
- id (INTEGER PRIMARY KEY AUTOINCREMENT)
- username (TEXT UNIQUE NOT NULL)
- password_hash (TEXT NOT NULL)
- role (TEXT DEFAULT 'developer')

## API Endpoints

### Core Bug Management
- POST /bugs/new - Create bug report
- GET /bugs - List all bugs
- GET /bugs/{id} - Get specific bug
- PATCH /bugs/{id} - Update bug
- DELETE /bugs/{id} - Delete bug

### HTML Interface
- GET / - Homepage dashboard
- GET /bugs/assign - Bug assignment form (HTML)
- POST /bugs/assign - Submit bug assignment

### Developer Management (BONUS)
- GET /developers - List all developers
- POST /developers - Add new developer

### Project Management
- GET /projects - List projects (thread-safe)
- POST /projects - Add project (admin only)

### Authentication & Security (BONUS)
- POST /login - User authentication (returns JWT)
- GET /protected - Protected endpoint (requires token)

### System Monitoring (BONUS)
- GET /health - Health check and system status
- POST /fix-admin - Reset admin password (development)

## Installation & Running

1. Ensure Rust 1.70+ installed
2. Create .env file with configuration
3. Run: cargo run
4. Server starts at http://127.0.0.1:8080
5. Database tables created automatically
6. Sample data populated on first run

## Configuration (.env file)