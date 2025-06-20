# Test Account Information

This application includes a pre-configured test account for development and testing purposes.

## Login Credentials

- **Email:** `test@example.com`
- **Password:** `password123`
- **User Name:** Test User

## Usage

1. When the application starts, you'll see the login window
2. Enter the test credentials above
3. Click "Sign In" to access the main application
4. Click "Logout" in the main window to return to the login screen

## Security Notice

⚠️ **This is a test account for development only!**

In a production environment, you should:
- Remove hardcoded credentials
- Implement proper authentication with a backend service
- Use secure password hashing (bcrypt, argon2, etc.)
- Implement proper session management
- Add rate limiting and other security measures

## Customizing the Test Account

To modify the test account, edit the `TEST_USER` constant in:
```
src-tauri/src/window_manager.rs
```

## Session Management

The app uses a simple file-based session token stored at:
```
src-tauri/session.token
```

This file is automatically created on login and deleted on logout.
It's included in `.gitignore` to prevent accidental commits.