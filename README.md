# Vero Core Contracts

On-chain GitHub PR verification for the Stellar ecosystem. Guardians — trusted off-chain validators — cast votes on registered tasks (pull requests). Once a configurable threshold is reached the task is marked done, creating a tamper-proof audit trail on Soroban.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   VeroContract                      │
│                                                     │
│  add_guardian(admin, guardian)                      │
│  register_task(admin, task_id)                      │
│  vote(guardian, task_id) ──► threshold check        │
│  get_task(task_id) ──► Task { id, votes, is_done }  │
└──────────────┬──────────────────────────────────────┘
               │ instance storage
       ┌───────┴────────┐
       │   DataKey      │
       │  Guardian(addr)│
       │  Task(u64)     │
       │  Voted(u64,addr│
       └────────────────┘
```

**Flow**

1. An admin registers a GitHub PR as a `Task` with a unique numeric ID.
2. The admin whitelists trusted validator addresses as guardians.
3. Each guardian calls `vote`. The contract rejects duplicates and non-guardians.
4. When `votes >= 3` the task's `is_done` flag flips to `true`.

---

## Modules

| Module | Responsibility |
|---|---|
| `types` | `Task`, `DataKey`, `ContractError` |
| `guardian` | Guardian registry with TTL-extended instance storage |
| `task` | Task registration and retrieval |
| `lib` | Public contract surface, `vote` orchestration |
| `events` | (reserved) on-chain event emission |

---

## Quick Start

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

---

## Code Snippets

### Register a task

```rust
// admin key must sign
client.register_task(&admin, &pr_number);
```

### Add a guardian

```rust
client.add_guardian(&admin, &validator_address);
```

### Cast a vote

```rust
// guardian key must sign; returns Err on duplicate or non-guardian
client.vote(&guardian, &pr_number)?;
```

### Query task state

```rust
let task: Task = client.get_task(&pr_number).unwrap();
assert!(task.is_done); // true once 3 votes are in
```

### Full test example

```rust
#[test]
fn test_three_votes_flips_is_done() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register_contract(None, VeroContract);
    let client = VeroContractClient::new(&env, &id);
    let admin = Address::generate(&env);

    let (g1, g2, g3) = (
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    );

    client.add_guardian(&admin, &g1);
    client.add_guardian(&admin, &g2);
    client.add_guardian(&admin, &g3);
    client.register_task(&admin, &42u64).unwrap();

    client.vote(&g1, &42u64).unwrap();
    client.vote(&g2, &42u64).unwrap();
    client.vote(&g3, &42u64).unwrap();

    assert!(client.get_task(&42u64).unwrap().is_done);
}
```

---

## Storage Design

All state lives in **instance storage** — scoped to the contract instance and extended with a 100 000-ledger TTL window on every guardian write. Keys are typed via the `DataKey` enum so there are no raw string collisions.

```rust
pub enum DataKey {
    Guardian(Address),   // bool — is this address a guardian?
    Task(u64),           // Task struct
    Voted(u64, Address), // bool — has this guardian voted on this task?
    Admin,               // reserved
}
```

---

## Error Codes

| Code | Meaning |
|---|---|
| `NotAuthorized (1)` | Caller is not a registered guardian or admin |
| `DuplicateVote (2)` | Guardian already voted on this task |

---

## License

Apache-2.0
