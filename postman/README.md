# PNAR World API - Postman Collection

A comprehensive Postman collection for testing the PNAR World API endpoints with pre-configured environment variables and example requests.

## 📋 Files Overview

- **`PNAR-API.postman_collection.json`** - Complete API collection with all endpoints
- **`PNAR-API.postman_environment.json`** - Environment variables for easy configuration
- **`README.md`** - Comprehensive usage guide (this file)
- **`demo-postman.sh`** - Demo script comparing Postman vs manual curl

## 🚀 Quick Start

### 1. Import Files

1. Open Postman
2. Click **"Import"** button (top left)
3. Select **File** tab
4. Choose both files:
   - `PNAR-API.postman_collection.json`
   - `PNAR-API.postman_environment.json`

### 2. Select Environment

1. Click the environment dropdown (top right)
2. Select **"PNAR API Environment"**

### 3. Update Base URL (if needed)

The environment is pre-configured for:

- **Base URL**: `http://localhost:8000/api/v1`
- **Test User**: `test@example.com` / `password123`

## 📚 API Endpoints

### 🏥 Health Check

**Test if the API is running**

```
GET /health
```

- **Expected Response**: `{"status": "healthy", "timestamp": "..."}`

### 🔐 Authentication

#### Register New User

```
POST /auth/register
Content-Type: application/json

{
  "email": "newuser@example.com",
  "password": "securepassword123",
  "full_name": "New User",
  "preferred_language": "en"
}
```

#### Login

```
POST /auth/login
Content-Type: application/json

{
  "email": "test@example.com",
  "password": "password123"
}
```

- **Saves JWT token automatically** for subsequent requests

#### Get Profile

```
GET /auth/profile
Authorization: Bearer {{auth_token}}
```

- Uses the token saved from login

### � Alphabets

#### Get All Alphabets

```
GET /alphabets
```

#### Convert Text

```
POST /alphabets/convert
Content-Type: application/json

{
  "text": "Hello World",
  "from_script": "latin",
  "to_script": "pnar"
}
```

### 📊 Analytics

#### List Analytics Records

```
GET /analytics?page=1&per_page=10&usage_type=click
```

- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `per_page`: Items per page (default: 20)
  - `usage_type`: Filter by usage type
  - `user_id`: Filter by user ID
  - `word_id`: Filter by word ID

#### Get Analytics Record by ID

```
GET /analytics/{id}
```

#### Create Analytics Record (Authenticated)

```
POST /analytics
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "word_id": "uuid",
  "usage_type": "click",
  "timestamp": "2025-08-28T15:00:00Z",
  "session_id": "session_123",
  "context_data": {
    "page": "dictionary",
    "action": "word_lookup"
  }
}
```

#### Create Anonymous Analytics Record

```
POST /analytics/anonymous
Content-Type: application/json

{
  "word_id": "uuid",
  "usage_type": "view",
  "timestamp": "2025-08-28T15:00:00Z",
  "session_id": "session_123",
  "context_data": {
    "page": "dictionary",
    "action": "word_view"
  }
}
```

#### Update Analytics Record

```
PUT /analytics/{id}
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "context_data": {
    "updated": true
  }
}
```

#### Delete Analytics Record

```
DELETE /analytics/{id}
Authorization: Bearer {{auth_token}}
```

#### Get Word Statistics

```
GET /analytics/words/{word_id}/stats
```

### � Books

#### List Public Books

```
GET /books?page=1&per_page=10
```

#### Get Book by ID

```
GET /books/{id}
```

#### Create Book (Authenticated)

```
POST /books
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "title": "Sample Book",
  "description": "A sample book for testing",
  "content": "Book content here...",
  "language": "en",
  "is_public": true
}
```

#### Update Book (Authenticated)

```
PUT /books/{id}
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "title": "Updated Book Title",
  "description": "Updated description",
  "is_public": false
}
```

#### Delete Book (Authenticated)

```
DELETE /books/{id}
Authorization: Bearer {{auth_token}}
```

#### Get My Books (Authenticated)

```
GET /books/my?page=1&per_page=10
Authorization: Bearer {{auth_token}}
```

### 📖 Dictionary

#### List Dictionary Entries

```
GET /dictionary?page=1&per_page=10
```

#### Get Dictionary Entry by ID

```
GET /dictionary/{id}
```

#### Search Dictionary

```
POST /dictionary/search
Content-Type: application/json

{
  "query": "hello",
  "language": "en",
  "limit": 10
}
```

#### Create Dictionary Entry (Admin)

```
POST /dictionary
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "pnar_word": "pnar_word",
  "english_word": "english word",
  "definition": "Definition of the word",
  "part_of_speech": "noun",
  "examples": ["Example 1", "Example 2"],
  "etymology": "Word origin",
  "phonetic": "phonetic spelling"
}
```

#### Update Dictionary Entry (Admin)

```
PUT /dictionary/{id}
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "definition": "Updated definition",
  "examples": ["Updated example"]
}
```

#### Delete Dictionary Entry (Admin)

```
DELETE /dictionary/{id}
Authorization: Bearer {{auth_token}}
```

### 🌐 Translations

#### List Translations

```
GET /translations?page=1&per_page=10
```

#### Get Translation by ID

```
GET /translations/{id}
```

#### Create Translation (Authenticated)

```
POST /translations
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "source_text": "Hello world",
  "target_text": "Pnar translation",
  "source_language": "en",
  "target_language": "pnar",
  "context": "Greeting"
}
```

#### Update Translation (Authenticated)

```
PUT /translations/{id}
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "target_text": "Updated Pnar translation",
  "context": "Updated greeting"
}
```

#### Delete Translation (Authenticated)

```
DELETE /translations/{id}
Authorization: Bearer {{auth_token}}
```

### 🤝 Contributions

#### Create Contribution (Authenticated)

```
POST /contributions
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "content_type": "dictionary_entry",
  "content_id": "uuid",
  "contribution_type": "create",
  "description": "Added new dictionary entry",
  "metadata": {
    "word": "new_word",
    "language": "pnar"
  }
}
```

#### Update Contribution (Authenticated)

```
PUT /contributions/{id}
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "status": "approved",
  "reviewer_notes": "Approved with minor corrections"
}
```

#### Delete Contribution (Authenticated)

```
DELETE /contributions/{id}
Authorization: Bearer {{auth_token}}
```

### 👥 Roles

#### List All Roles

```
GET /roles
```

#### List Assignable Roles (Authenticated)

```
GET /roles/assignable
Authorization: Bearer {{auth_token}}
```

#### List Manageable Roles (Authenticated)

```
GET /roles/manageable
Authorization: Bearer {{auth_token}}
```

### 🔔 Notifications

#### List Notifications (Authenticated)

```
GET /notifications?page=1&per_page=10
Authorization: Bearer {{auth_token}}
```

#### Get Notification by ID (Authenticated)

```
GET /notifications/{id}
Authorization: Bearer {{auth_token}}
```

#### Get Unread Count (Authenticated)

```
GET /notifications/unread-count
Authorization: Bearer {{auth_token}}
```

#### Create Notification (Authenticated)

```
POST /notifications
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "title": "Test Notification",
  "message": "This is a test notification",
  "notification_type": "info",
  "recipient_id": "uuid"
}
```

#### Update Notification (Authenticated)

```
PUT /notifications/{id}
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "is_read": true
}
```

#### Mark Notification as Read (Authenticated)

```
PUT /notifications/{id}/read
Authorization: Bearer {{auth_token}}
```

#### Mark All Notifications as Read (Authenticated)

```
PUT /notifications/mark-all-read
Authorization: Bearer {{auth_token}}
```

#### Delete Notification (Authenticated)

```
DELETE /notifications/{id}
Authorization: Bearer {{auth_token}}
```

## 🔧 Environment Variables

The collection uses the following environment variables:

- **`base_url`**: Base URL for the API (default: `http://localhost:8000/api/v1`)
- **`auth_token`**: JWT token for authenticated requests (auto-populated on login)
- **`user_id`**: User ID for filtering (optional)
- **`word_id`**: Word ID for filtering (optional)
- **`book_id`**: Book ID for operations (optional)

## � Testing Tips

### 1. Authentication Flow

1. **Register** a new user or **Login** with existing credentials
2. The JWT token is automatically saved to `auth_token` variable
3. All subsequent requests will use this token

### 2. Using UUIDs

Replace placeholder UUIDs (`123e4567-e89b-12d3-a456-426614174000`) with actual IDs from your database or API responses.

### 3. Testing Different Scenarios

- **Public endpoints**: No authentication required
- **Protected endpoints**: Require `Authorization: Bearer {{auth_token}}` header
- **Admin endpoints**: Require admin privileges (dictionary management)

### 4. Response Validation

Most endpoints return data in this format:

```json
{
  "success": true,
  "data": { ... },
  "message": "Operation successful"
}
```

## � Troubleshooting

### Common Issues

1. **401 Unauthorized**: Check if you're logged in and token is valid
2. **403 Forbidden**: You may not have required permissions
3. **404 Not Found**: Check the endpoint URL and parameters
4. **500 Internal Server Error**: Check server logs for details

### Token Expiration

If your token expires, simply run the **Login** request again to get a new token.

## � Collection Features

- ✅ **Complete API Coverage**: All endpoints included
- ✅ **Auto Token Management**: JWT tokens saved automatically
- ✅ **Environment Variables**: Easy configuration
- ✅ **Request Examples**: Sample data for all endpoints
- ✅ **Query Parameters**: Pagination and filtering examples
- ✅ **Error Handling**: Proper status codes and error messages

## 🔄 Version History

- **v2.0.0**: Complete rewrite with all endpoints, improved documentation
- **v1.0.0**: Basic collection with core endpoints

---

For more information about the PNAR World API, check the main project documentation.
