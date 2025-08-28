# Postman Setup for PNAR World API

## 🚀 Quick Setup

1. **Install Postman** (if you don't have it)

   - Download from: https://www.postman.com/downloads/

2. **Import the Collection & Environment**

   - Open Postman
   - Click "Import" button
   - Select both files:
     - `PNAR-API.postman_collection.json`
     - `PNAR-API.postman_environment.json`

3. **Select Environment**

   - In the top-right dropdown, select "PNAR API Environment"

4. **Start Testing!**
   - Run requests in this order:
     1. Health Check → Get Health Status
     2. Alphabets → Get All Alphabets
     3. Books → Get Books
     4. Authentication → Register User (if needed)
     5. Authentication → Login (saves token automatically)
     6. Authentication → Get Profile (uses saved token)

## 📋 What's Included

### Collection Features:

- ✅ **Health Check** - Monitor API status
- ✅ **Alphabets API** - Get Pnar characters
- ✅ **Books API** - Browse public books
- ✅ **Authentication** - Register, login, get profile
- ✅ **Auto Token Management** - Login automatically saves JWT token
- ✅ **Environment Variables** - Easy configuration

### Environment Variables:

- `base_url` - API base URL (default: http://localhost:8000/api/v1)
- `auth_token` - JWT token (auto-filled after login)
- `test_email` - Test user email
- `test_password` - Test user password

## 🎯 Usage Tips

1. **Start the API server first:**

   ```bash
   cargo run
   ```

2. **Test the flow:**

   - Health check should always work
   - Alphabets should always work
   - Books should always work
   - Auth endpoints require valid credentials

3. **Authentication Flow:**

   - Register a user (or use existing)
   - Login to get JWT token
   - Token is automatically saved and used for authenticated requests

4. **Customize for your environment:**
   - Edit the `base_url` variable if your server runs on different port
   - Modify test credentials as needed

## 🔧 Troubleshooting

- **Connection refused?** Make sure the API server is running on localhost:8000
- **Auth errors?** Try registering a new user or check credentials
- **Token expired?** Login again to refresh the token

## 📚 Alternative Tools

If you prefer not to use Postman:

- `simple-api-docs.html` - Web-based tester
- `test-api-simple.sh` - Quick bash script
- `quick-test.sh` - Full-featured script

Happy testing! 🎉</content>
<parameter name="filePath">/Users/armegochylla/Projects/panr-online/pnar-world-backend-rust/Postman-README.md
