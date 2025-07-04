# Complete Bug Tracker Testing Guide - CSC1106 Quiz Requirements
# Group 14 (g14_practical)

## 📋 **Testing Checklist for All Quiz Requirements**

This document provides comprehensive testing commands to verify all CSC1106 practical quiz requirements are properly implemented for the Rust-based web application bug tracking system using Actix Web framework and SQLite database with enhanced security using bcrypt password hashing and JWT authentication.

## Configuration (.env file)
```
DATABASE_URL=sqlite:bugs.db
JWT_SECRET=your-super-secret-jwt-key-bugtrack2025
BCRYPT_COST=12
```

## Default Users & Data
- Admin: username=admin, password=admin123 (Role: admin)
- Sample Developers: John Developer, Jane Smith, Bob Wilson
- Sample Projects: Frontend Development, Backend API, Mobile App

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

## Core Features Implemented

### 1. Bug Report Creation (POST /bugs/new)
- Accepts JSON with title, description, reported_by, severity
- Stores in SQLite database with auto-generated ID
- Returns created bug record with assigned bug_id as JSON
- Comprehensive validation and error handling

### 2. Project List State Management (GET/POST /projects)
- Thread-safe in-memory project storage using Arc<RwLock<Vec<Project>>>
- GET /projects - returns project list as JSON
- POST /projects - adds new project (admin use only)
- Pre-loaded with sample projects

### 3. User Login with Password Hashing (POST /login)
- bcrypt hashing with fixed salt "bugtrack2025" as specified
- Password hash = Hash(SALT + password) using bcrypt
- Returns success/failure status with JWT session token
- Pre-created admin user: admin/admin123

### 4. Bug Assignment with HTML Templates (GET/POST /bugs/assign)
- Dynamic HTML generation with bug and developer listings
- Bug assignment form at GET /bugs/assign
- Form submission updates bug record with developer_id
- Confirmation page with error handling for invalid IDs

### 5. Full CRUD for Bugs
- POST /bugs/new - Create new bug report
- GET /bugs - List all bugs as JSON
- GET /bugs/:id - Retrieve specific bug (404 if not found)
- PATCH /bugs/:id - Update bug details
- DELETE /bugs/:id - Delete bug (404 if missing)
- Proper validation, error handling, and HTTP status codes

## 🚀 BONUS FEATURES IMPLEMENTED

### 6. Professional Homepage Dashboard
- Interactive homepage at GET / with system overview
- API documentation with all endpoints listed
- Quick navigation links to main features
- Copy-paste ready test commands
- Modern responsive web design

### 7. JWT Token-Based Authentication System
- Industry-standard JSON Web Tokens for session management
- Token verification middleware for protected endpoints
- GET /protected - demonstrates secure API access
- Token expiration handling (24-hour validity)
- Secure Authorization header parsing (Bearer token format)

### 8. Developer Management System
- GET /developers - List all developers with full details
- POST /developers - Add new developer to system
- Auto-populated sample developers (John, Jane, Bob)
- Integration with bug assignment workflow

### 9. Enhanced Security Features
- bcrypt password hashing (industry standard vs basic SHA)
- Salted password storage with fixed salt as required
- SQL injection prevention via parameterized queries
- Input validation and sanitization on all endpoints
- Structured error responses without information leakage

### 10. Dynamic HTML Form Generation
- Real-time bug and developer data in assignment forms
- Dropdown selections populated from database
- User-friendly interface with styling and validation
- Success/failure confirmation pages with navigation

### 11. Professional API Architecture
- Structured JSON responses with success/error status
- Consistent response format across all endpoints
- Comprehensive error handling with appropriate HTTP codes
- RESTful API design principles

### 12. Database Schema Enhancements
- Users table with role-based access control
- Proper foreign key relationships (bugs -> developers)
- Data integrity constraints and validation
- Automatic sample data population on startup

### 13. Health Monitoring & Debugging
- GET /health - System status endpoint for monitoring
- Service information and version details
- Admin password reset functionality (POST /fix-admin)
- Comprehensive logging throughout application

### 14. Advanced State Management
- Thread-safe concurrent access using Arc<RwLock>
- In-memory project storage as specified
- Atomic operations for data consistency
- Scalable architecture for multiple users

### 15. Production-Ready Error Handling
- Graceful error responses for all failure scenarios
- Input validation with helpful error messages
- Database error handling and recovery
- Proper HTTP status codes (200, 201, 400, 404, 500)

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
3. Run: export DATABASE_URL="sqlite://$(pwd)/bugs.db"
        export JWT_SECRET="replace_with_a_long_random_string"
3. Run: cargo run
4. Server starts at http://127.0.0.1:8080
5. Database tables created automatically
6. Sample data populated on first run

---

## 🚀 **Pre-Testing Setup**

### **1. Start the Server**
```bash
cargo run
```

**Expected Output:**
```
🚀 Starting Bug Tracker Server...
✅ Database connection established and tables verified
🌐 Server starting at http://127.0.0.1:8080
📖 Default admin credentials: admin/admin123
```

### **2. Verify Server Health**
```bash
curl http://localhost:8080/health
```

**Expected Response:**
```json
{"status":"healthy","service":"Bug Tracker API","version":"1.0.0"}
```

---

## 📝 **REQUIREMENT 1: Bug Report Creation Endpoint**

### **Test 1.1: Create New Bug Report (POST /bugs/new)**

```bash
curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"Login Button Not Working\",\"description\":\"Users cannot click the login button on the homepage\",\"reported_by\":\"john.doe@company.com\",\"severity\":\"High\"}"
```

**Expected Response:**
```json
{
  "success": true,
  "message": "Bug created successfully",
  "data": {
    "id": 1,
    "title": "Login Button Not Working",
    "description": "Users cannot click the login button on the homepage",
    "reported_by": "john.doe@company.com",
    "severity": "High",
    "developer_id": null
  },
  "bug_id": 1
}
```

### **Test 1.2: Create Multiple Bug Reports**

```bash
# Bug #2
curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"Database Connection Error\",\"description\":\"Application crashes when accessing user profiles\",\"reported_by\":\"jane.smith@company.com\",\"severity\":\"Critical\"}"

# Bug #3
curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"UI Alignment Issue\",\"description\":\"Buttons are misaligned on mobile devices\",\"reported_by\":\"bob.wilson@company.com\",\"severity\":\"Medium\"}"

# Bug #4
curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"Search Function Slow\",\"description\":\"Search takes more than 10 seconds to return results\",\"reported_by\":\"alice.johnson@company.com\",\"severity\":\"Low\"}"
```

### **Test 1.3: Validation Testing**

**Empty Title (Should Fail):**
```bash
curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"\",\"description\":\"Test\",\"reported_by\":\"test@test.com\",\"severity\":\"Low\"}"
```

**Expected Response:**
```json
{"success":false,"message":"Title is required","data":null,"bug_id":null}
```

### **✅ Requirement 1 Verification:**
- ✅ Accepts JSON with title, description, reported_by, severity
- ✅ Stores in SQLite database with auto-generated bug_id
- ✅ Returns created bug record as JSON on success
- ✅ Proper error handling for invalid data

---

## 🗂️ **REQUIREMENT 2: Project List State Management**

### **Test 2.1: Get Project List (GET /projects)**

```bash
curl http://localhost:8080/projects
```

**Expected Response:**
```json
[
  {
    "id": 1,
    "name": "Frontend Development",
    "description": "User interface and UX improvements",
    "active": true
  },
  {
    "id": 2,
    "name": "Backend API", 
    "description": "Server-side development and database optimization",
    "active": true
  },
  {
    "id": 3,
    "name": "Mobile App",
    "description": "iOS and Android application development", 
    "active": true
  }
]
```

### **Test 2.2: Add New Project (POST /projects)**

```bash
curl -X POST http://localhost:8080/projects -H "Content-Type: application/json" -d "{\"name\":\"DevOps Pipeline\",\"description\":\"CI/CD and deployment automation\"}"
```

**Expected Response:**
```json
{
  "id": 4,
  "name": "DevOps Pipeline",
  "description": "CI/CD and deployment automation", 
  "active": true
}
```

### **Test 2.3: Thread-Safety Testing (Multiple Concurrent Requests)**

**Run these commands quickly in succession:**
```bash
curl -X POST http://localhost:8080/projects -H "Content-Type: application/json" -d "{\"name\":\"Project Alpha\",\"description\":\"Alpha testing project\"}" &

curl -X POST http://localhost:8080/projects -H "Content-Type: application/json" -d "{\"name\":\"Project Beta\",\"description\":\"Beta testing project\"}" &

curl -X POST http://localhost:8080/projects -H "Content-Type: application/json" -d "{\"name\":\"Project Gamma\",\"description\":\"Gamma testing project\"}" &

wait
```

**Verify all projects were added:**
```bash
curl http://localhost:8080/projects
```

### **Test 2.4: Validation Testing**

**Empty Project Name (Should Fail):**
```bash
curl -X POST http://localhost:8080/projects -H "Content-Type: application/json" -d "{\"name\":\"\",\"description\":\"Test project\"}"
```

**Expected Response:**
```json
"Project name is required"
```

### **✅ Requirement 2 Verification:**
- ✅ Thread-safe in-memory storage using Arc<RwLock<Vec<Project>>>
- ✅ GET /projects returns project list as JSON
- ✅ POST /projects adds new project (admin use)
- ✅ State persists across requests until server restart

---

## 🔐 **REQUIREMENT 3: User Login with Password Hashing**

### **Test 3.1: Fix Admin Password (Setup)**

```bash
curl -X POST http://localhost:8080/fix-admin
```

**Expected Response:**
```json
"Admin password fixed"
```

### **Test 3.2: Successful Login (POST /login)**

```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"username\":\"admin\",\"password\":\"admin123\"}"
```

**Expected Response:**
```json
{
  "status": "success",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "message": "Login successful"
}
```

### **Test 3.3: Failed Login Attempts**

**Wrong Password:**
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"username\":\"admin\",\"password\":\"wrongpassword\"}"
```

**Expected Response:**
```json
{
  "status": "failure",
  "token": null,
  "message": "Invalid username or password"
}
```

**Wrong Username:**
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"username\":\"wronguser\",\"password\":\"admin123\"}"
```

**Expected Response:**
```json
{
  "status": "failure",
  "token": null,
  "message": "Invalid username or password"
}
```

**Empty Credentials:**
```bash
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"username\":\"\",\"password\":\"\"}"
```

**Expected Response:**
```json
{
  "status": "failure",
  "token": null,
  "message": "Username and password are required"
}
```

### **Test 3.4: Token Validation (Bonus Feature)**

**Use token from successful login above:**
```bash
curl -H "Authorization: Bearer YOUR_TOKEN_HERE" http://localhost:8080/protected
```

**Expected Response:**
```json
{
  "message": "Access granted!",
  "user": "admin",
  "role": "admin",
  "expires": 1751684620
}
```

### **✅ Requirement 3 Verification:**
- ✅ Password hashing with fixed salt "bugtrack2025"
- ✅ Hash calculation: Hash(SALT + password) using bcrypt
- ✅ Returns "success" status with session token on match
- ✅ Returns "failure" status on incorrect credentials

---

## 🎯 **REQUIREMENT 4: Bug Assignment with HTML Templates**

### **Test 4.1: Bug Assignment Form (GET /bugs/assign)**

**Visit in Browser:**
```
http://localhost:8080/bugs/assign
```

**Or test with curl:**
```bash
curl http://localhost:8080/bugs/assign
```

**Expected:** HTML page with:
- Dropdown list of available bugs
- Dropdown list of available developers
- Assignment form

### **Test 4.2: Submit Bug Assignment (POST /bugs/assign)**

**First, get available developers:**
```bash
curl http://localhost:8080/developers
```

**Then assign bug #1 to developer #1:**
```bash
curl -X POST http://localhost:8080/bugs/assign -H "Content-Type: application/x-www-form-urlencoded" -d "bug_id=1&developer_id=1"
```

**Expected Response:** HTML confirmation page showing successful assignment

### **Test 4.3: Verify Assignment in Database**

```bash
curl http://localhost:8080/bugs/1
```

**Expected Response:**
```json
{
  "id": 1,
  "title": "Login Button Not Working",
  "description": "Users cannot click the login button on the homepage",
  "reported_by": "john.doe@company.com", 
  "severity": "High",
  "developer_id": 1
}
```

### **Test 4.4: Invalid Assignment Testing**

**Non-existent Bug:**
```bash
curl -X POST http://localhost:8080/bugs/assign -H "Content-Type: application/x-www-form-urlencoded" -d "bug_id=999&developer_id=1"
```

**Expected Response:** HTML error page with "Bug or developer not found"

**Non-existent Developer:**
```bash
curl -X POST http://localhost:8080/bugs/assign -H "Content-Type: application/x-www-form-urlencoded" -d "bug_id=1&developer_id=999"
```

**Expected Response:** HTML error page with "Bug or developer not found"

### **✅ Requirement 4 Verification:**
- ✅ HTML template rendering using Tera
- ✅ Bug assignment form at GET /bugs/assign
- ✅ Form submission updates bug record with developer_id
- ✅ Confirmation page display
- ✅ Error handling for invalid developer or bug IDs

---

## 🔄 **REQUIREMENT 5: Full CRUD for Bugs**

### **Test 5.1: CREATE - Bug Creation (Already Tested)**
*See Requirement 1 tests above*

### **Test 5.2: READ - List All Bugs (GET /bugs)**

```bash
curl http://localhost:8080/bugs
```

**Expected Response:** JSON array of all bugs with filtering support

### **Test 5.3: READ - Get Specific Bug (GET /bugs/:id)**

```bash
curl http://localhost:8080/bugs/1
```

**Expected Response:**
```json
{
  "id": 1,
  "title": "Login Button Not Working",
  "description": "Users cannot click the login button on the homepage",
  "reported_by": "john.doe@company.com",
  "severity": "High", 
  "developer_id": 1
}
```

**Test Non-existent Bug:**
```bash
curl http://localhost:8080/bugs/999
```

**Expected Response:**
```json
"Bug not found"
```

### **Test 5.4: UPDATE - Update Bug Details (PATCH /bugs/:id)**

```bash
curl -X PATCH http://localhost:8080/bugs/1 -H "Content-Type: application/json" -d "{\"id\":1,\"title\":\"Login Button Fixed\",\"description\":\"Bug has been resolved - login button now works\",\"reported_by\":\"john.doe@company.com\",\"severity\":\"High\",\"developer_id\":1}"
```

**Expected Response:** Updated bug record with new values

**Test Invalid Bug Update:**
```bash
curl -X PATCH http://localhost:8080/bugs/999 -H "Content-Type: application/json" -d "{\"id\":999,\"title\":\"Non-existent\",\"description\":\"Test\",\"reported_by\":\"test\",\"severity\":\"Low\",\"developer_id\":null}"
```

**Expected Response:**
```json
"Bug not found"
```

### **Test 5.5: DELETE - Delete Bug (DELETE /bugs/:id)**

```bash
curl -X DELETE http://localhost:8080/bugs/4
```

**Expected Response:**
```json
"Bug deleted successfully"
```

**Test Delete Non-existent Bug:**
```bash
curl -X DELETE http://localhost:8080/bugs/999
```

**Expected Response:**
```json
"Bug not found"
```

**Verify Deletion:**
```bash
curl http://localhost:8080/bugs/4
```

**Expected Response:**
```json
"Bug not found"
```

### **✅ Requirement 5 Verification:**
- ✅ POST /bugs/new - Create new bug report
- ✅ GET /bugs - List all bugs as JSON with filtering
- ✅ GET /bugs/:id - Retrieve specific bug (404 if not found)
- ✅ PATCH /bugs/:id - Update bug details
- ✅ DELETE /bugs/:id - Delete bug (404 if missing)
- ✅ Proper validation, error handling, and HTTP status codes

---

## 🏆 **BONUS FEATURES TESTING**

### **Developer Management**

**List Developers:**
```bash
curl http://localhost:8080/developers
```

**Add New Developer:**
```bash
curl -X POST http://localhost:8080/developers -H "Content-Type: application/json" -d "{\"name\":\"Sarah Connor\"}"
```

### **Homepage Dashboard**

**Visit Homepage:**
```
http://localhost:8080/
```

### **Protected Endpoint (JWT Authentication)**

**Access with valid token:**
```bash
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://localhost:8080/protected
```

**Access without token:**
```bash
curl http://localhost:8080/protected
```

---

## 📊 **Complete System Test Sequence**

### **Run this complete test to verify all functionality:**

```bash
# 1. Health Check
curl http://localhost:8080/health

# 2. Create bugs
curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"Test Bug 1\",\"description\":\"Description 1\",\"reported_by\":\"test1@test.com\",\"severity\":\"High\"}"

curl -X POST http://localhost:8080/bugs/new -H "Content-Type: application/json" -d "{\"title\":\"Test Bug 2\",\"description\":\"Description 2\",\"reported_by\":\"test2@test.com\",\"severity\":\"Medium\"}"

# 3. List all bugs
curl http://localhost:8080/bugs

# 4. Get projects
curl http://localhost:8080/projects

# 5. Add project
curl -X POST http://localhost:8080/projects -H "Content-Type: application/json" -d "{\"name\":\"Test Project\",\"description\":\"Testing project\"}"

# 6. Login
curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d "{\"username\":\"admin\",\"password\":\"admin123\"}"

# 7. Get developers
curl http://localhost:8080/developers

# 8. Update bug
curl -X PATCH http://localhost:8080/bugs/1 -H "Content-Type: application/json" -d "{\"id\":1,\"title\":\"Updated Bug\",\"description\":\"Updated description\",\"reported_by\":\"test1@test.com\",\"severity\":\"Critical\",\"developer_id\":1}"

# 9. Delete bug
curl -X DELETE http://localhost:8080/bugs/2

# 10. Verify final state
curl http://localhost:8080/bugs
```

---

## ✅ **Final Verification Checklist**

### **Core Requirements:**
- [ ] Bug creation with JSON response ✓
- [ ] Thread-safe project state management ✓
- [ ] Password hashing with salt "bugtrack2025" ✓
- [ ] HTML bug assignment form ✓
- [ ] Full CRUD operations for bugs ✓

### **Technical Requirements:**
- [ ] SQLite database storage ✓
- [ ] Actix Web framework ✓
- [ ] Proper error handling ✓ 
- [ ] HTTP status codes ✓
- [ ] JSON API responses ✓

### **Bonus Features:**
- [ ] JWT authentication system ✓
- [ ] Developer management ✓
- [ ] Professional homepage ✓
- [ ] Health monitoring ✓
- [ ] Enhanced security ✓

---

## 🎯 **Expected Test Results Summary**

After running all tests, you should have:

1. **4+ bugs created** (some may be deleted)
2. **6+ projects** (3 default + additions)
3. **3 developers** (John, Jane, Bob)
4. **1 admin user** (admin/admin123)
5. **Working authentication** with JWT tokens
6. **Functional HTML forms** for bug assignment
7. **Complete CRUD operations** working
8. **Thread-safe state management** verified

---

## 🚀 **Ready for Submission!**

If all tests pass, your bug tracker system meets and exceeds all CSC1106 practical quiz requirements with professional bonus features!