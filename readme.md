ipfs-rs/
│── Cargo.toml         # Dependencies & package configuration
│── README.md          # Documentation
│
├── src/
│   ├── main.rs        # Entry point of the program
│   │
│   ├── lib.rs         # Central module re-exports
│   │
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
