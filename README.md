# Soulgraph Rust SDK

[![Crates.io](https://img.shields.io/crates/v/soulgraph.svg)](https://crates.io/crates/soulgraph)
[![Documentation](https://docs.rs/soulgraph/badge.svg)](https://docs.rs/soulgraph)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/soulgraph/soulgraph-rs/workflows/CI/badge.svg)](https://github.com/soulgraph/soulgraph-rs/actions)

A Rust SDK for interacting with the Soulgraph API, enabling the creation and management of AI personalities with rich traits, memories, and behaviors. Soulgraph helps developers create more human-like AI interactions by managing personality traits, emotional responses, and behavioral patterns.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)
- [Development](#development)
- [Testing](#testing)
- [Security](#security)
- [License](#license)
- [Contact](#contact)

## Features

- 🔑 **Authentication & Security**

  - Simple API key authentication
  - Secure request handling
  - Rate limiting support

- 🛠️ **Developer Experience**

  - Intuitive builder patterns
  - Type-safe API interactions
  - Comprehensive error handling
  - Async/await support

- 🧠 **Personality Management**

  - Create and modify AI personalities
  - Define behavioral traits
  - Set communication styles
  - Manage personality evolution

- 💭 **Memory & Learning**

  - Emotional memory system
  - Experience tracking
  - Behavioral adaptation
  - Context awareness

- 🤖 **Relationship Modeling**

  - Define interaction boundaries
  - Manage relationship dynamics
  - Track relationship evolution
  - Set interaction styles

- 🗣️ **Communication Control**
  - Voice and tone management
  - Communication style patterns
  - Dynamic response adaptation
  - Contextual awareness

## Installation

Add Soulgraph to your `Cargo.toml`:

```toml
[dependencies]
soulgraph = "0.1.0"
```

For optional features:

```toml
[dependencies]
soulgraph = { version = "0.1.0", features = ["async-std", "memory-cache"] }
```

## Quick Start

Here's a simple example to get you started with Soulgraph:

```rust
use soulgraph::{Soulgraph, Soul, personality::{Personality, traits::TraitBuilder}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = Soulgraph::builder()
        .api_key(std::env::var("SOULGRAPH_API_KEY")?)
        .base_url("https://api.soulgraph.com")
        .build();

    // Create traits for the personality
    let helpful = TraitBuilder::new("helpful")
        .strength(0.9)
        .add_expression_rule("always seeks to assist")
        .build();

    let knowledgeable = TraitBuilder::new("knowledgeable")
        .strength(0.8)
        .add_expression_rule("provides detailed explanations")
        .build();

    // Create a personality
    let personality = Personality::builder()
        .name("Assistant")
        .add_trait(helpful)
        .add_trait(knowledgeable)
        .build()
        .unwrap();

    // Create a soul with the personality
    let soul = Soul::builder()
        .personality(personality)
        .build();

    println!("Created soul with ID: {}", soul.id.unwrap_or_default());

    Ok(())
}
```

## Usage Examples

### Creating a Complex Personality

```rust
use soulgraph::{Personality, Voice, Relationship};

let personality = Personality::builder()
    .name("Military Trainer")
    .traits(vec!["disciplined", "motivating", "strict"])
    .voice(Voice::default())  // Uses the military-style voice
    .relationship(Relationship::default())  // Sets mentor-like relationship
    .build();
```

### Managing Memory and Experiences

```rust
use soulgraph::memories::{Memory, MemoryBuilder};

let memory = MemoryBuilder::new("First successful training session".to_string())
    .importance_score(0.8)
    .emotional_signature(EmotionalSignature {
        id: None,
        valence: 0.8,
        intensity: 0.9,
    })
    .build();
```

## API Documentation

For detailed API documentation, visit [docs.rs/soulgraph](https://docs.rs/soulgraph) or run:

```bash
cargo doc --open
```

Key concepts:

- `Soulgraph`: Main client for API interactions
- `Soul`: Core entity representing an AI personality
- `Personality`: Defines behavioral traits and characteristics
- `Memory`: Manages experiences and emotional responses
- `Relationship`: Controls interaction dynamics
- `Voice`: Defines communication style and patterns

## Contributing

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the tests (`cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Setup

1. Clone the repository
2. Install Rust (if you haven't already) via [rustup](https://rustup.rs/)
3. Build the project: `cargo build`
4. Run tests: `cargo test`

### Code Style

- Follow the Rust API guidelines
- Use the standard Rust formatting (`cargo fmt`)
- Ensure all code passes `cargo clippy`
- Add tests for new functionality
- Document public APIs with doc comments

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

- Open an issue for bug reports or feature requests
- Join our [Discord community](https://discord.gg/soulgraph) for discussions
- Check out our [documentation](https://docs.soulgraph.com) for guides and examples

## Acknowledgments

- Built with ❤️ for the AI development community
