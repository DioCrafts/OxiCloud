# 19 - Database Transactions

OxiCloud uses explicit transactions on PostgreSQL to guarantee data integrity. All transactional operations follow ACID properties:

- **Atomicity** -- all-or-nothing. If any part fails, the entire transaction fails.
- **Consistency** -- the database moves from one valid state to another.
- **Isolation** -- concurrent transactions behave as if sequential.
- **Durability** -- once committed, the transaction survives system failures.

## Implementation

The **with_transaction** helper wraps the standard pattern: begin, execute, commit on success, rollback on error.

### Transaction Utility

Located in `src/infrastructure/repositories/pg/transaction_utils.rs`:

```rust
/// Helper function to execute database operations in a transaction
pub async fn with_transaction<F, T, E>(
    pool: &Arc<PgPool>,
    operation_name: &str,
    operation: F,
) -> Result<T, E>
where
    F: for<'c> FnOnce(&'c mut Transaction<'_, Postgres>) -> futures::future::BoxFuture<'c, Result<T, E>>,
    E: From<SqlxError> + std::fmt::Display
{ ... }
```

This function takes a connection pool and a closure with operations, handles begin/commit/rollback automatically, and provides detailed logging of the transaction lifecycle.

### Repository Usage Example

```rust
// Creación de un usuario con transacción explícita
async fn create_user(&self, user: User) -> UserRepositoryResult<User> {
    with_transaction(
        &self.pool,
        "create_user",
        |tx| {
            Box::pin(async move {
                // Operación principal - insertar usuario
                sqlx::query("INSERT INTO auth.users ...")
                    .bind(...)
                    .execute(&mut **tx)
                    .await?;

                // Operaciones adicionales dentro de la misma transacción
                // ...

                Ok(user_clone)
            })
        }
    ).await
}
```

## Use Cases

### UserPgRepository

1. **User creation** -- guarantees all insert operations are atomic. Allows related operations (like permission setup) to be bundled.

2. **User update** -- ensures modifications apply fully or not at all. Supports combined operations like profile info and preference updates.

### SessionPgRepository

1. **Session creation** -- inserts the session and updates the user's last-access timestamp in a single transaction. Keeps sessions and user data consistent.

2. **Session revocation** -- ensures revoking one or all sessions for a user is atomic. Allows logging security events within the same transaction.

## Isolation Levels

OxiCloud supports different transaction isolation levels via **with_transaction_isolation**:

```rust
// Ejemplo de uso con nivel de aislamiento específico
with_transaction_isolation(
    &pool,
    "operacion_critica",
    sqlx::postgres::PgIsolationLevel::Serializable,
    |tx| { ... }
).await
```

Available isolation levels:

1. **Read Committed** (default) -- guarantees reads see only committed data. Does not prevent non-repeatable or phantom reads.

2. **Repeatable Read** -- guarantees consistent reads throughout the transaction. Prevents non-repeatable reads but not phantom reads.

3. **Serializable** -- highest isolation level. Transactions behave as if executed serially. Can cause serialization errors that require retry.

## Best Practices

1. **Transaction duration** -- keep transactions as short as possible. Avoid long-running operations inside them.

2. **Error handling** -- errors inside a transaction trigger automatic rollback. Use proper logging to diagnose failures.

3. **Transaction boundaries** -- define clearly where transactions begin and end. Group related operations into a single transaction.

4. **Appropriate isolation** -- use the lowest isolation level that fits the use case. Consider serializable for critical operations with conflict potential.

## Benefits

- **Data integrity** -- ACID guarantees for complex operations, prevents inconsistent states.
- **Error handling** -- automatic rollback on failure, predictable behavior.
- **Safe concurrency** -- proper handling of simultaneous operations, prevents race conditions.
- **Performance** -- fewer round-trips to the database, batch operations for better efficiency.

## Performance Considerations

Transactions add some overhead. Performance can be affected by:

- Transaction duration
- Isolation level
- Number of affected records
- Lock contention
