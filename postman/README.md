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
  "username": "newuser"
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

### 📚 Alphabets

#### Get All Alphabets

```
GET /alphabets
```

#### Get Alphabet by ID

```
GET /alphabets/{id}
```

#### Create Alphabet

```
POST /alphabets
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "name": "Pnar Alphabet",
  "description": "Traditional Pnar writing system",
  "characters": ["ꯀ", "ꯁ", "ꯂ", "ꯃ"]
}
```

### 📖 Books

#### Get All Books

```
GET /books
```

#### Get Book by ID

```
GET /books/{id}
```

#### Create Book

```
POST /books
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "title": "Pnar Language Guide",
  "author": "Pnar Community",
  "description": "A comprehensive guide to Pnar language",
  "language": "en",
  "content": "Book content here..."
}
```

### 🔄 Translation

#### Request Translation

```
POST /translations
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "source_text": "Hello World",
  "source_language": "en",
  "target_language": "pnar",
  "context": "greeting"
}
```

#### Get Translation by ID

```
GET /translations/{id}
```

#### Get User's Translations

```
GET /translations/user/{user_id}
```

### 📊 Analytics

#### Create Analytics Event

```
POST /analytics
Authorization: Bearer {{auth_token}}
Content-Type: application/json

{
  "event_type": "word_lookup",
  "word_id": "uuid-here",
  "metadata": {
    "source": "dictionary",
    "confidence": 0.95
  }
}
```

#### Get Analytics Records

```
GET /analytics?page=1&per_page=20
```

## 🎯 Testing Workflow

### 1. Health Check

- Run **Health Check → Get Health Status**
- Should return: `{"status": "healthy"}`

### 2. Authentication

- Run **Authentication → Register User** (optional)
- Run **Authentication → Login**
  - This saves the JWT token automatically
- Run **Authentication → Get Profile**
  - Should work with the saved token

### 3. Core Features

- **Alphabets → Get All Alphabets**
- **Books → Get Books**
- **Translation → Request Translation** (requires auth)
- **Analytics → Create Analytics** (requires auth)

## 🔧 Environment Variables

The environment file includes:

| Variable        | Default Value                  | Description          |
| --------------- | ------------------------------ | -------------------- |
| `base_url`      | `http://localhost:8000/api/v1` | API base URL         |
| `auth_token`    | _(auto-filled)_                | JWT token from login |
| `test_email`    | `test@example.com`             | Test user email      |
| `test_password` | `password123`                  | Test user password   |

## 🛠️ Advanced Usage

### Custom Environment

Create multiple environments for different setups:

1. **Development**: `http://localhost:8000/api/v1`
2. **Staging**: `https://staging-api.example.com/api/v1`
3. **Production**: `https://api.pnarworld.com/api/v1`

### Running Tests

Use Postman's built-in test runner:

```javascript
// Example test in Postman
pm.test('Status code is 200', function () {
  pm.response.to.have.status(200);
});

pm.test('Response has required fields', function () {
  var jsonData = pm.response.json();
  pm.expect(jsonData).to.have.property('status');
});
```

### Data Generation

For testing with different data:

```javascript
// Generate random test data
var randomEmail = 'test' + Math.floor(Math.random() * 1000) + '@example.com';
pm.environment.set('test_email', randomEmail);
```

## 🚨 Common Issues

### 401 Unauthorized

- **Solution**: Run login request first to get JWT token
- **Check**: Ensure `Authorization: Bearer {{auth_token}}` header is present

### 404 Not Found

- **Solution**: Check base URL in environment
- **Check**: Ensure API is running on correct port

### Connection Refused

- **Solution**: Start the development server
- **Command**: `./dev.sh`

## 📖 Additional Resources

- [Postman Documentation](https://learning.postman.com/)
- [PNAR World API Docs](../API-README.md)
- [Development Guide](../DEV-README.md)

## 🎬 Demo Script

Run the demo script to see the difference between using Postman vs manual curl commands:

```bash
# Run the demo comparison
./demo-postman.sh
```

This script shows:
- ✅ **Postman workflow**: Import → Configure → Test instantly
- ❌ **Manual curl workflow**: Write commands → Handle auth → Parse responses
- 📊 **Comparison**: Why Postman is more efficient for API development

## 🤝 Contributing

When adding new API endpoints:

1. Add the request to the Postman collection
2. Include proper headers and authentication
3. Add example request/response data
4. Update this README with the new endpoint

---

**Happy API Testing! 🎉**
