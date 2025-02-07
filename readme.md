# **ipfs-rs**

A Rust implementation for an IPFS-like system with basic functionalities such as Content Identifier (CID) handling, storage management, networking, pinning, and more. This project aims to provide a decentralized file-sharing platform and a system to manage, resolve, and retrieve content via CIDs.

---

## **Features**

- **Content Identifier (CID) Handling**:
  - Generates and resolves CIDs for content.
  
- **Storage**:
  - Local block storage to manage file slices and Merkle-DAG structures.
  
- **Networking**:
  - P2P networking with libp2p & DHT for peer discovery and file sharing.

- **Pinning**:
  - Pinning files to ensure persistence.

- **Gateway**:
  - HTTP gateway for serving files over the web.

- **Configuration Management**:
  - Load and store configuration settings.

---

## **Project Structure**

```
ipfs-rs/
│── Cargo.toml         # Dependencies & package configuration
│── README.md          # Documentation
│
├── src/
│   ├── main.rs        # Entry point of the program
│   ├── lib.rs         # Central module re-exports
│   ├── cid/           # Content Identifier (CID) handling
│   │   ├── mod.rs     # CID module entry
│   │   ├── generator.rs  # Generates CIDs from content
│   │   ├── resolver.rs   # Resolves CID to actual content
│   │
│   ├── storage/       # Local block storage
│   │   ├── mod.rs     # Storage module entry
│   │   ├── block.rs   # Block storage handler
│   │   ├── dag.rs     # Merkle-DAG structure
│   │
│   ├── network/       # P2P Networking (libp2p & DHT)
│   │   ├── mod.rs     # Networking module entry
│   │   ├── dht.rs     # Distributed Hash Table for peer discovery
│   │   ├── bitswap.rs # File sharing mechanism
│   │   ├── peer.rs    # Peer connection management
│   │
│   ├── pinning/       # Pinning & persistence
│   │   ├── mod.rs     # Pinning module entry
│   │   ├── pin.rs     # Handles local file persistence
│   │
│   ├── gateway/       # HTTP gateway to serve files
│   │   ├── mod.rs     # HTTP API module entry
│   │   ├── server.rs  # HTTP server implementation
│   │
│   ├── config/        # Configuration management
│   │   ├── mod.rs     # Config module entry
│   │   ├── settings.rs # Load/store config settings
│   │
│   ├── utils/         # Utility functions
│   │   ├── mod.rs     # Utilities module entry
│   │   ├── hashing.rs # Helper functions for hashing
│   │
│── tests/             # Integration tests
│   ├── cid_tests.rs   # Tests for CID functionality
│   ├── dht_tests.rs   # Tests for DHT networking
│   ├── storage_tests.rs # Tests for block storage
```

---

## **Dependencies**

- `cid`: Rust crate for working with Content Identifiers.
- `surrealdb`: A fast, key-value store for storing file slices and metadata.
- `libp2p`: Networking library for peer-to-peer communication.
- `serde`, `serde_json`: Serialization/deserialization for data formats.
- `tokio`: Asynchronous runtime for handling tasks concurrently.

---

## **Getting Started**

### **Installation**

1. **Clone the Repository:**

```bash
git clone https://github.com/manaslaud/ipfs-rs.git
cd ipfs-rs
```

2. **Install Rust Dependencies:**

Make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed. If not, you can install it with the following:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. **Build the Project:**

```bash
cargo build
```

4. **Run the Project:**

You can run the project with:

```bash
cargo run
```


## **Testing**

To run the tests, use the following command:

```bash
cargo test
```

Tests are located in the `tests` directory, with separate files for CID functionality, DHT networking, and storage operations.

---

## **Contributing**

1. **Fork the Repository**
2. **Clone your fork:**

```bash
git clone https://github.com/yourusername/ipfs-rs.git
```

3. **Create a branch for your feature or fix:**

```bash
git checkout -b new-feature
```

4. **Make your changes** and **commit** them:

```bash
git commit -am "Add new feature"
```

5. **Push your changes** to your fork:

```bash
git push origin new-feature
```

6. **Create a Pull Request**.

---

## **License**

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

### **Contact**

For any inquiries or issues, feel free to open an issue or contact me directly.
[Linkedin](https://www.linkedin.com/in/manaslaud/)
