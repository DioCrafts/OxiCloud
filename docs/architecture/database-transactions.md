# Database Transactions

OxiCloud uses explicit PostgreSQL transactions for multi-step operations that must either commit together or fail together.

## ACID Guarantees

- Atomicity: all work succeeds or the entire transaction rolls back
- Consistency: constraints and invariants remain valid before and after commit
- Isolation: concurrent work behaves predictably
- Durability: committed writes survive process and system failures

## Transaction Helper

The PostgreSQL repositories use a helper like `with_transaction` to standardize the transaction lifecycle:

```rust
pub async fn with_transaction<F, T, E>(
    pool: &Arc<PgPool>,
    operation_name: &str,
    operation: F,
) -> Result<T, E>
where
    F: for<'c> FnOnce(&'c mut Transaction<'_, Postgres>) -> futures::future::BoxFuture<'c, Result<T, E>>,
    E: From<SqlxError> + std::fmt::Display,
{ /* ... */ }
```

That wrapper handles begin, commit, rollback, and lifecycle logging so repository code can focus on the actual domain operation.

## Common Use Cases

### User management

Transactions keep related user changes together, such as creating a user and attaching the dependent records required for a valid account.

### Session management

Session creation and session revocation can update multiple tables in a single logical step, which avoids stale or mismatched security state.

### File and folder workflows

Moves, renames, trash operations, and other multi-step metadata changes rely on transactions so the tree stays consistent.

## Isolation Levels

OxiCloud can use different isolation levels depending on the operation.

| Level | Use case |
| --- | --- |
| `Read Committed` | Default application work |
| `Repeatable Read` | Stable reads during a longer unit of work |
| `Serializable` | Highest safety for conflict-prone critical operations |

Higher isolation can introduce retries or contention, so it should be reserved for the few flows that need it.

## Best Practices

- Keep transactions short
- Avoid heavy I/O inside a transaction when possible
- Group only operations that must commit together
- Choose the lowest isolation level that preserves correctness
- Log and surface rollback causes clearly

## Why It Matters

- Prevents partial metadata updates
- Keeps concurrent user activity predictable
- Reduces race conditions in critical operations
- Makes failures recoverable and easier to reason about

## Related Pages

- [Storage Safety](/architecture/file-system-safety)
- [Internal Architecture](/architecture/)