---
title: API Design Best Practices
tags: [reference, api, best-practices]
author: Tech Lead
category: development
---

# API Design Best Practices

## RESTful Principles

### 1. Use Nouns, Not Verbs

✅ **Good:**
```
GET /users
POST /users
GET /users/123
```

❌ **Bad:**
```
GET /getUsers
POST /createUser
```

### 2. Use Plural Nouns

✅ **Good:** `/orders` instead of `/order`

### 3. Use HTTP Methods Correctly

- `GET` - Retrieve resource
- `POST` - Create resource
- `PUT` - Update resource (full)
- `PATCH` - Update resource (partial)
- `DELETE` - Remove resource

## Response Format

### Success Response
```json
{
  "data": {
    "id": 123,
    "name": "John Doe"
  },
  "meta": {
    "timestamp": "2024-01-15T10:00:00Z"
  }
}
```

### Error Response
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": [
      {
        "field": "email",
        "issue": "Must be a valid email address"
      }
    ]
  }
}
```

## Pagination

Use cursor-based pagination for large datasets:

```
GET /users?cursor=abc123&limit=20
```

Response:
```json
{
  "data": [...],
  "pagination": {
    "next_cursor": "def456",
    "has_more": true
  }
}
```

## Versioning

Include API version in URL:
```
/api/v1/users
/api/v2/users
```

## Security

- Always use HTTPS in production
- Implement rate limiting
- Use authentication tokens
- Validate all inputs
- Sanitize outputs

## References

- [REST API Tutorial](https://restfulapi.net/)
- [MDN Web Docs - HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)

Related: [[api-redesign]], [[architecture]]

#reference #api #best-practices #development
