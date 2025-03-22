# Brand Validator On-Chain

A Solana program that enables secure product validation and ownership tracking on the blockchain.

## Overview

This program provides a secure way to:
- Add products to the blockchain
- Validate product authenticity
- Track product ownership
- Manage product lifecycle

## Features

- Product registration with unique IDs
- Ownership verification
- Purchase tracking
- Secure deletion capabilities
- Company authorization system

## Prerequisites

- Rust 1.70.0 or later
- Solana CLI tools
- Anchor Framework

## Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/brand-validator-on-chain.git
cd brand-validator-on-chain
```

2. Build the program:
```bash
anchor build
```

3. Deploy to your preferred Solana network:
```bash
anchor deploy
```

## Usage

### Adding a Product
```rust
add_product(ctx, product_id: String)
```

### Validating a Product
```rust
validate_product(ctx) -> (String, bool)
```

### Purchasing a Product
```rust
buy_product(ctx, product_id: String)
```

### Deleting a Product
```rust
delete_product(ctx)
```

## Security

- Only authorized companies can add products
- Product ownership is verified on-chain
- Purchase status is immutable once confirmed
- Secure account closure for product deletion

## License

MIT License - See LICENSE file for details

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## Support

For support, please open an issue in the GitHub repository. 