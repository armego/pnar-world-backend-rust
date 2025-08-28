#!/bin/bash

# PNAR API - Postman Workflow Demo
# Shows how simple API testing is with Postman vs Swagger

echo "🎯 PNAR World API - Postman vs Swagger Comparison"
echo "=================================================="
echo ""

echo "📋 WITH POSTMAN (Recommended):"
echo "------------------------------"
echo "✅ Import collection: PNAR-API.postman_collection.json"
echo "✅ Import environment: PNAR-API.postman_environment.json"
echo "✅ Click 'Health Check' → Send → See results instantly"
echo "✅ Click 'Login' → Edit credentials → Send → Token auto-saved"
echo "✅ Click 'Get Profile' → Uses saved token automatically"
echo ""

echo "📋 WITHOUT POSTMAN (Manual curl):"
echo "----------------------------------"
echo "❌ Copy endpoint URL from docs"
echo "❌ Manually add headers"
echo "❌ Manually format JSON body"
echo "❌ Manually save/manage JWT tokens"
echo "❌ No automatic environment switching"
echo ""

echo "🚀 QUICK DEMO - Testing all endpoints:"
echo "---------------------------------------"

BASE_URL="http://localhost:8000/api/v1"

# Test Health
echo "1. Health Check..."
curl -s "$BASE_URL/health" | jq '.status' 2>/dev/null || echo "   (Server not running?)"

# Test Alphabets
echo "2. Alphabets..."
curl -s "$BASE_URL/alphabets" | jq 'length' 2>/dev/null || echo "   (Server not running?)"

# Test Books
echo "3. Books..."
curl -s "$BASE_URL/books" | jq '.data | length' 2>/dev/null || echo "   (Server not running?)"

# Test Auth (will fail but shows proper error)
echo "4. Auth (demo)..."
curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"demo@example.com","password":"demo"}' | jq '.error.message' 2>/dev/null || echo "   (Server not running?)"

echo ""
echo "🎉 With Postman, all this is ONE-CLICK!"
echo ""
echo "📖 See Postman-README.md for setup instructions"
echo "📦 Files: PNAR-API.postman_collection.json + PNAR-API.postman_environment.json"</content>
<parameter name="filePath">/Users/armegochylla/Projects/panr-online/pnar-world-backend-rust/demo-postman.sh
