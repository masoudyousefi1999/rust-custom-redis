# Redis Clone (Rust)

A lightweight Redis-inspired in-memory database written in Rust.

This project was built as a learning exercise to explore:

- TCP networking
- Redis protocol basics (RESP)
- Command parsing
- Error handling in Rust
- Ownership and borrowing
- In-memory data structures
- TTL (Time To Live) expiration

The server communicates using the Redis Serialization Protocol (RESP), allowing it to be used directly with `redis-cli`.

---

## Features

### Supported Commands

| Command | Description |
|----------|-------------|
| `PING` | Returns `PONG` |
| `SET key value` | Stores a value |
| `GET key` | Retrieves a value |
| `DEL key` | Deletes a key |
| `EXISTS key` | Checks whether a key exists |
| `KEYS` | Lists all keys |
| `EXPIRE key seconds` | Sets a TTL on a key |
| `TTL key` | Returns remaining TTL |
| `COMMNADS` | Returns list of available commands |

---

## Running the Server

Clone the repository:

```bash
git clone <repository-url>
cd redis-clone
```

Run the server:

```bash
cargo run
```

The server starts on:

```text
127.0.0.1:3030
```

---

## Connecting with redis-cli

This project is designed to work directly with Redis CLI.

Open another terminal and connect:

```bash
redis-cli -p 3030
```

If everything is running correctly, you can start executing commands immediately.

---

## Examples

### Ping

```redis
PING
```

Response:

```text
PONG
```

---

### Set a Value

```redis
SET name masoud
```

Response:

```text
OK
```

---

### Get a Value

```redis
GET name
```

Response:

```text
masoud
```

---

### Check Existence

```redis
EXISTS name
```

Response:

```text
(integer) 1
```

---

### Delete a Key

```redis
DEL name
```

Response:

```text
(integer) 1
```

---

### Set Expiration

```redis
SET user masoud
EXPIRE user 60
```

Response:

```text
(integer) 1
```

---

### Get Remaining TTL

```redis
TTL user
```

Response:

```text
(integer) 42
```

---

### List Keys

```redis
KEYS
```

Response:

```text
1) "user"
2) "name"
```

---

## Implementation Notes

Current implementation includes:

- Single-threaded TCP server
- RESP-compatible responses
- In-memory storage using `HashMap`
- Lazy expiration strategy for TTL keys
- Custom error handling with Rust `Result` and `?`

---

## Limitations

This is a learning project and not intended for production use.

Current limitations include:

- No persistence
- No replication
- No transactions
- No authentication
- No pub/sub
- No clustering
- No concurrent client handling
- Partial RESP support

---

## Future Improvements

Potential future additions:

- Full RESP parser
- Persistence (RDB/AOF style)
- Concurrent client support
- Background expiration cleanup
- Additional Redis commands
- Benchmarking and performance improvements

---

## Built With

- Rust
- TCP Sockets
- Redis CLI
- RESP Protocol

---

## License

This project is available for educational and personal use.