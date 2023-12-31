

## Architecture
- websocket server (Rust): handles all connections between clients and rooms
- HTTP server:
    - room manager: manages all rooms (creating, deleting, updating) in the application
    - user manager: manages all users (creating, deleting, updating)

- message manager: on both HTTP and websocket server. manages creating, deleting, updating messages. used to store incoming messages from websockets on the http server, and used to retrieve all messages when a user opens the chat room using the rest API