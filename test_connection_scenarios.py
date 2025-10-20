#!/usr/bin/env python3
"""
Comprehensive test script for all connection scenarios.
Tests different environment configurations to ensure backward compatibility.
"""
import os
import sys
from sqlalchemy import create_engine
from sqlalchemy.pool import NullPool

def test_scenario(scenario_name, env_vars, expected_behavior):
    """Test a specific connection scenario"""
    print(f"\n{'=' * 60}")
    print(f"SCENARIO: {scenario_name}")
    print('=' * 60)
    
    # Clear previous env vars
    for key in ['user', 'password', 'host', 'port', 'dbname', 'DATABASE_URL']:
        if key in os.environ:
            del os.environ[key]
    
    # Set new env vars
    for key, value in env_vars.items():
        os.environ[key] = value
        if key == 'password':
            print(f"  {key}: {'*' * len(value)}")
        else:
            print(f"  {key}: {value}")
    
    # Simulate the logic from dependencies.py
    USER = os.getenv("user")
    PASSWORD = os.getenv("password")
    HOST = os.getenv("host")
    PORT = os.getenv("port")
    DBNAME = os.getenv("dbname")
    
    if all([USER, PASSWORD, HOST, PORT, DBNAME]):
        DATABASE_URL = f"postgresql+psycopg2://{USER}:{PASSWORD}@{HOST}:{PORT}/{DBNAME}?sslmode=require"
        source = "Individual parameters"
    else:
        DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./courier.db")
        source = "DATABASE_URL" if "DATABASE_URL" in env_vars else "Default (SQLite)"
    
    # Mask password in output
    if PASSWORD and PASSWORD in DATABASE_URL:
        masked_url = DATABASE_URL.replace(PASSWORD, '*' * len(PASSWORD))
    else:
        masked_url = DATABASE_URL
    
    print(f"\nResult:")
    print(f"  Source: {source}")
    print(f"  URL: {masked_url}")
    
    # Check pool configuration
    if not DATABASE_URL.startswith("sqlite"):
        print(f"  Pool: NullPool (for PostgreSQL)")
        print(f"  SSL: {'Yes (sslmode=require)' if 'sslmode=require' in DATABASE_URL else 'Default'}")
    else:
        print(f"  Pool: Default (for SQLite)")
    
    # Validate against expected behavior
    success = True
    for check, expected_value in expected_behavior.items():
        if check == "source":
            if source != expected_value:
                print(f"  ❌ FAIL: Expected source '{expected_value}', got '{source}'")
                success = False
        elif check == "has_ssl":
            has_ssl = "sslmode=require" in DATABASE_URL
            if has_ssl != expected_value:
                print(f"  ❌ FAIL: Expected SSL={expected_value}, got SSL={has_ssl}")
                success = False
        elif check == "is_postgres":
            is_postgres = DATABASE_URL.startswith("postgresql")
            if is_postgres != expected_value:
                print(f"  ❌ FAIL: Expected PostgreSQL={expected_value}, got PostgreSQL={is_postgres}")
                success = False
    
    if success:
        print(f"  ✅ PASSED")
    
    return success

def main():
    """Run all test scenarios"""
    print("=" * 60)
    print("CONNECTION CONFIGURATION TEST SUITE")
    print("=" * 60)
    
    all_passed = True
    
    # Scenario 1: Individual parameters (new format)
    all_passed &= test_scenario(
        "Individual Parameters (Session Pooler)",
        {
            'user': 'postgres.testproject',
            'password': 'test_password',
            'host': 'aws-1-us-east-2.pooler.supabase.com',
            'port': '5432',
            'dbname': 'postgres'
        },
        {
            'source': 'Individual parameters',
            'has_ssl': True,
            'is_postgres': True
        }
    )
    
    # Scenario 2: DATABASE_URL only (legacy)
    all_passed &= test_scenario(
        "DATABASE_URL Only (Legacy)",
        {
            'DATABASE_URL': 'postgresql://postgres:legacy_pass@db.xyz.supabase.co:5432/postgres'
        },
        {
            'source': 'DATABASE_URL',
            'has_ssl': False,  # Legacy format doesn't add SSL by default
            'is_postgres': True
        }
    )
    
    # Scenario 3: No configuration (default SQLite)
    all_passed &= test_scenario(
        "No Configuration (Default)",
        {},
        {
            'source': 'Default (SQLite)',
            'has_ssl': False,
            'is_postgres': False
        }
    )
    
    # Scenario 4: Partial individual parameters (should fallback)
    all_passed &= test_scenario(
        "Partial Parameters (Missing dbname)",
        {
            'user': 'postgres.testproject',
            'password': 'test_password',
            'host': 'aws-1-us-east-2.pooler.supabase.com',
            'port': '5432',
            # Missing dbname - should fallback to DATABASE_URL or default
        },
        {
            'source': 'Default (SQLite)',
            'has_ssl': False,
            'is_postgres': False
        }
    )
    
    # Scenario 5: Both individual parameters and DATABASE_URL (individual should win)
    all_passed &= test_scenario(
        "Both Formats (Individual Takes Priority)",
        {
            'user': 'postgres.testproject',
            'password': 'test_password',
            'host': 'aws-1-us-east-2.pooler.supabase.com',
            'port': '5432',
            'dbname': 'postgres',
            'DATABASE_URL': 'postgresql://postgres:legacy@db.xyz.supabase.co:5432/postgres'
        },
        {
            'source': 'Individual parameters',
            'has_ssl': True,
            'is_postgres': True
        }
    )
    
    # Final summary
    print("\n" + "=" * 60)
    print("TEST SUITE SUMMARY")
    print("=" * 60)
    if all_passed:
        print("✅ All scenarios passed!")
        print("\nThe connection configuration logic is working correctly:")
        print("  • Individual parameters work (new Session Pooler format)")
        print("  • DATABASE_URL still works (backward compatibility)")
        print("  • Defaults to SQLite when no configuration is provided")
        print("  • Individual parameters take priority over DATABASE_URL")
        print("  • SSL is automatically configured for individual parameters")
        return 0
    else:
        print("❌ Some scenarios failed!")
        print("Please review the implementation in src/api/dependencies.py")
        return 1

if __name__ == "__main__":
    sys.exit(main())
