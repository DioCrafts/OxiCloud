# 18 - Database Migrations

OxiCloud uses versioned SQL files to manage database schema changes. The migration system ensures changes are versioned, trackable, consistently applied across environments, reproducible, and independent of application code.

## Directory Structure

```
OxiCloud/
├── db/
│   └── schema.sql             # Main database schema
├── src/
    ├── bin/
    │   └── migrate.rs         # CLI tool for running migrations
    ├── common/
    │   └── db.rs              # Database connection with schema verification
```

> The schema is currently applied from `db/schema.sql` at application startup (when it detects the `auth` tables don't exist). The `migrations/` directory doesn't exist yet, but `src/bin/migrate.rs` is ready to use sqlx migrations once the `migrations` feature is enabled.

## Naming Conventions

Migration files follow this format: `YYYYMMDDHHMMSS_brief_description.sql`

- `YYYYMMDDHHMMSS` -- timestamp that guarantees correct ordering (year, month, day, hour, minute, second)
- `brief_description` -- short description of the migration purpose
- `.sql` -- SQL file extension

## Running Migrations

Migrations run via a dedicated CLI tool:

```bash
cargo run --bin migrate --features migrations
```

This command:
1. Connects to the database configured in the environment
2. Looks for migrations in the `/migrations/` directory
3. Compares applied migrations against available ones
4. Sequentially executes pending migrations
5. Records applied migrations in a control table

## Creating New Migrations

To create a new migration:

1. Create a new file in `migrations/` following the naming convention
2. Define the SQL changes in the file
3. Make sure the changes are compatible with the current schema version
4. Run the migrations

Example migration structure:

```sql
-- Migración: Añadir tabla de etiquetas
-- Descripción: Crea la tabla para almacenar etiquetas de archivos y sus relaciones

-- Crear tabla de etiquetas
CREATE TABLE IF NOT EXISTS auth.tags (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#3498db',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, name)
);

-- Crear índices
CREATE INDEX IF NOT EXISTS idx_tags_user_id ON auth.tags(user_id);

-- Tabla de relación entre archivos y etiquetas
CREATE TABLE IF NOT EXISTS auth.file_tags (
    id SERIAL PRIMARY KEY,
    tag_id INTEGER NOT NULL REFERENCES auth.tags(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tag_id, file_id)
);

-- Comentarios de documentación
COMMENT ON TABLE auth.tags IS 'Almacena etiquetas definidas por usuarios';
COMMENT ON TABLE auth.file_tags IS 'Relación muchos-a-muchos entre archivos y etiquetas';
```

## Best Practices

1. **Incremental migrations** -- each migration should represent one atomic, coherent change.

2. **Idempotent migrations** -- use commands that can run multiple times without errors (e.g., `CREATE TABLE IF NOT EXISTS`).

3. **Forward-only migrations** -- design migrations to move forward, not roll back. If you need to undo a change, create a new migration.

4. **Forward compatibility** -- migrations must be compatible with both the existing code and the code about to be deployed.

5. **Test before deploying** -- test migrations in a production-like environment before applying them.

6. **Documentation** -- document the purpose and key changes of each migration with comments inside the SQL file.

## Troubleshooting

### Checking Migration State

OxiCloud includes startup-time detection to verify which migrations have been applied:

```rust
// Desde src/common/db.rs
let migration_check = sqlx::query("SELECT EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'auth' AND tablename = 'users')")
    .fetch_one(&pool)
    .await;
    
match migration_check {
    Ok(row) => {
        let tables_exist: bool = row.get(0);
        if !tables_exist {
            tracing::warn!("Las tablas de la base de datos no existen. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
        }
    },
    Err(_) => {
        tracing::warn!("No se pudo verificar el estado de las migraciones. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
    }
}
```

### Common Issues

1. **Database connection error** -- verify the connection URL in the **DATABASE_URL** environment variable.

2. **Migration conflicts** -- if a migration fails, check the error messages for conflicts with the existing schema.

3. **Insufficient permissions** -- make sure the database user has permissions to create schemas, tables, and indexes.

4. **"Admin already exists" error** -- if you get this error when trying to register an admin user, follow these steps:

   a. Connect to the PostgreSQL container:
   ```bash
   # Find the container
   docker ps
   # Example: oxicloud-postgres-1
   docker exec -it oxicloud-postgres-1 bash
   ```

   b. Connect to the database:
   ```bash
   psql -U postgres -d oxicloud
   ```

   c. Set the schema and delete the existing admin user:
   ```sql
   SET search_path TO auth;
   DELETE FROM auth.users WHERE username = 'admin';
   ```

   d. Verify the deletion:
   ```sql
   SELECT username, email, role FROM auth.users;
   ```

   e. Exit PostgreSQL:
   ```sql
   \q
   exit
   ```

   f. You can now register a new admin user through the OxiCloud interface.

   Alternatively, use the provided script:
   ```bash
   cat scripts/reset_admin.sql | docker exec -i oxicloud-postgres-1 psql -U postgres -d oxicloud
   ```

## Benefits of Migration-Based Approach

- **Separation of concerns** -- migrations live separately from application code.
- **Automation** -- simplifies deployment automation and CI/CD.
- **Change history** -- provides a clear history of schema evolution.
- **Collaboration** -- lets multiple developers contribute schema changes in an orderly way.
- **Multiple environments** -- guarantees identical database structures across dev, test, and production.
