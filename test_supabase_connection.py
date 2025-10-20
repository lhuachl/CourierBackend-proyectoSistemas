#!/usr/bin/env python3
"""
Test script for Supabase Session Pooler connection configuration.
This script validates that the database connection is properly configured.
"""
from sqlalchemy import create_engine
from sqlalchemy.pool import NullPool
from dotenv import load_dotenv
import os

# Load environment variables from .env
load_dotenv()

# Fetch variables
USER = os.getenv("user")
PASSWORD = os.getenv("password")
HOST = os.getenv("host")
PORT = os.getenv("port")
DBNAME = os.getenv("dbname")

print("=" * 60)
print("SUPABASE SESSION POOLER CONNECTION TEST")
print("=" * 60)

# Check if all required variables are present
print("\n--- Environment Variables ---")
if all([USER, PASSWORD, HOST, PORT, DBNAME]):
    print(f"✅ user: {USER}")
    print(f"✅ password: {'*' * len(PASSWORD) if PASSWORD else 'NOT SET'}")
    print(f"✅ host: {HOST}")
    print(f"✅ port: {PORT}")
    print(f"✅ dbname: {DBNAME}")
    
    # Construct the SQLAlchemy connection string
    DATABASE_URL = f"postgresql+psycopg2://{USER}:{PASSWORD}@{HOST}:{PORT}/{DBNAME}?sslmode=require"
    
    print("\n--- Connection String ---")
    # Mask the password in the output
    masked_url = DATABASE_URL.replace(PASSWORD, '*' * len(PASSWORD))
    print(f"DATABASE_URL: {masked_url}")
    
    # Create the SQLAlchemy engine with NullPool
    # This is recommended for Supabase Session Pooler to disable client-side pooling
    print("\n--- Creating Engine ---")
    engine = create_engine(DATABASE_URL, poolclass=NullPool)
    print("✅ Engine created with NullPool (recommended for Session Pooler)")
    
    # Test the connection
    print("\n--- Testing Connection ---")
    try:
        with engine.connect() as connection:
            result = connection.execute("SELECT 1")
            print("✅ Connection successful!")
            print("   Database is accessible via Supabase Session Pooler")
    except Exception as e:
        print(f"❌ Failed to connect: {e}")
        print("\nTroubleshooting:")
        print("1. Verify your password is correct")
        print("2. Ensure your Supabase project is active (not paused)")
        print("3. Check that Session Pooler is enabled in Supabase")
        print("4. Verify the connection parameters match your Supabase project")
else:
    print("❌ Missing environment variables!")
    print("\nPlease configure the following in your .env file:")
    if not USER:
        print("  - user")
    if not PASSWORD:
        print("  - password")
    if not HOST:
        print("  - host")
    if not PORT:
        print("  - port")
    if not DBNAME:
        print("  - dbname")
    print("\nSee .env.example for reference.")

print("\n" + "=" * 60)
print("TEST COMPLETED")
print("=" * 60)
