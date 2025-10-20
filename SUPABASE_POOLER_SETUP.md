# Supabase Session Pooler Setup Guide

This document explains the changes made to support Supabase Session Pooler connection with SQLAlchemy.

## What Changed

### 1. New Environment Variable Format

The `.env.example` file now uses individual connection parameters instead of a single `DATABASE_URL`:

```bash
# Old format (still supported for backward compatibility)
DATABASE_URL=postgresql://postgres:password@db.project.supabase.co:5432/postgres

# New format (recommended for Session Pooler)
user=postgres.hlmngthhnvbdvbrxukqy
password=[YOUR_PASSWORD]
host=aws-1-us-east-2.pooler.supabase.com
port=5432
dbname=postgres
```

### 2. Updated Connection Logic in `src/api/dependencies.py`

The application now:
- Reads individual connection parameters (user, password, host, port, dbname)
- Builds the connection URL automatically with `sslmode=require`
- Uses `NullPool` for PostgreSQL connections (recommended for Supabase Session Pooler)
- Falls back to `DATABASE_URL` if individual parameters are not provided (backward compatibility)

### 3. SQLAlchemy NullPool Configuration

When connecting to PostgreSQL (including Supabase), the application now uses `NullPool` instead of the default connection pool. This is important because:

- Supabase Session Pooler already manages connections on the server side
- Using client-side pooling (SQLAlchemy's default) + server-side pooling (Supabase) can cause conflicts
- `NullPool` disables client-side pooling, letting Supabase handle all connection management
- This is the [recommended configuration](https://docs.sqlalchemy.org/en/20/core/pooling.html#switching-pool-implementations) by SQLAlchemy for pooled connections

## How to Configure

### Step 1: Find Your Supabase Session Pooler Credentials

1. Log in to [Supabase](https://app.supabase.com/)
2. Select your project
3. Navigate to: **Settings** → **Database** → **Connection Pooling**
4. Select **Session** mode (recommended for SQLAlchemy)
5. Copy the connection parameters:
   - **Host**: `aws-X-us-east-Y.pooler.supabase.com`
   - **Port**: `5432`
   - **Database**: `postgres`
   - **User**: `postgres.[project-ref]` (e.g., `postgres.hlmngthhnvbdvbrxukqy`)
   - **Password**: Your database password

### Step 2: Create Your `.env` File

```bash
# Copy the example file
cp .env.example .env

# Edit .env with your actual values
nano .env
```

Your `.env` should look like this:

```bash
# Database Configuration (Supabase PostgreSQL - Session Pooler)
user=postgres.hlmngthhnvbdvbrxukqy
password=your_actual_password_here
host=aws-1-us-east-2.pooler.supabase.com
port=5432
dbname=postgres

# JWT Security Configuration
SECRET_KEY=your-secret-key-change-this-to-a-random-string
JWT_ALGORITHM=HS256
ACCESS_TOKEN_EXPIRE_MINUTES=30
```

### Step 3: Test Your Connection

Run the included test script to verify your configuration:

```bash
python test_supabase_connection.py
```

Expected output:
```
============================================================
SUPABASE SESSION POOLER CONNECTION TEST
============================================================

--- Environment Variables ---
✅ user: postgres.hlmngthhnvbdvbrxukqy
✅ password: *****************
✅ host: aws-1-us-east-2.pooler.supabase.com
✅ port: 5432
✅ dbname: postgres

--- Connection String ---
DATABASE_URL: postgresql+psycopg2://postgres.hlmngthhnvbdvbrxukqy:*****************@aws-1-us-east-2.pooler.supabase.com:5432/postgres?sslmode=require

--- Creating Engine ---
✅ Engine created with NullPool (recommended for Session Pooler)

--- Testing Connection ---
✅ Connection successful!
   Database is accessible via Supabase Session Pooler
```

### Step 4: Start Your Application

```bash
python main.py
```

## Backward Compatibility

The old `DATABASE_URL` format still works! If you prefer to use it, simply set:

```bash
DATABASE_URL=postgresql://postgres:password@db.project.supabase.co:5432/postgres
```

The application will automatically detect this and use it. However, we recommend using the new format with Session Pooler for better performance.

## Benefits of Session Pooler

1. **Better Performance**: Connection pooling at the server level
2. **Resource Efficiency**: No need for client-side connection pooling
3. **Scalability**: Handles many concurrent connections efficiently
4. **Recommended by Supabase**: Specifically designed for ORMs like SQLAlchemy

## Troubleshooting

### Connection Fails with "could not translate host name"

- Verify you're using the pooler host (e.g., `aws-1-us-east-2.pooler.supabase.com`)
- Check that Session Pooler is enabled in Supabase settings

### Connection Fails with "password authentication failed"

- Double-check your password in the `.env` file
- Ensure there are no extra spaces or quotes around the password

### Project is Paused

Supabase free tier pauses projects after 7 days of inactivity:
1. Go to your Supabase dashboard
2. Click "Restore" or "Unpause"
3. Wait a few minutes for the project to wake up

### SSL/TLS Errors

The connection automatically includes `sslmode=require`. If you need to change this:
- Modify the connection string building logic in `src/api/dependencies.py`
- Change `?sslmode=require` to `?sslmode=prefer` or another mode

## Additional Resources

- [Supabase Connection Pooling Documentation](https://supabase.com/docs/guides/database/connecting-to-postgres#connection-pooler)
- [SQLAlchemy Pooling Documentation](https://docs.sqlalchemy.org/en/20/core/pooling.html)
- [Supabase Performance Best Practices](https://supabase.com/docs/guides/database/database-advisor)
