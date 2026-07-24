# References

## OKF Specification

- [Open Knowledge Format](https://github.com/open-knowledge-format/spec) — Human-readable, git-versionable knowledge representation

## Key Libraries

| Library | Purpose | Link |
|---------|---------|------|
| `ignore` | Filesystem traversal with `.gitignore` support | [GitHub](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) |
| `saphyr` | YAML 1.2 parser with Serde integration | [GitHub](https://github.com/saphyr-rs/saphyr) |
| `pulldown-cmark` | Streaming CommonMark parser | [GitHub](https://github.com/raphlinus/pulldown-cmark) |
| `rusqlite` | SQLite wrapper with FTS5 support | [GitHub](https://github.com/rusqlite/rusqlite) |
| `blake3` | Fast cryptographic hashing | [GitHub](https://github.com/BLAKE3-team/BLAKE3) |
| `clap` | Command-line argument parsing | [GitHub](https://github.com/clap-rs/clap) |
| `rmcp` | Model Context Protocol SDK | [GitHub](https://github.com/modelcontextprotocol/rust-sdk) |
| `tokio` | Async runtime | [GitHub](https://github.com/tokio-rs/tokio) |
| `thiserror` | Derive error types | [GitHub](https://github.com/dtolnay/thiserror) |
| `anyhow` | Application error handling | [GitHub](https://github.com/dtolnay/anyhow) |
| `miette` | Diagnostic error reporting | [GitHub](https://github.com/zkat/miette) |
| `tracing` | Structured logging | [GitHub](https://github.com/tokio-rs/tracing) |
| `schemars` | JSON Schema generation | [GitHub](https://github.com/keats/schemars) |
| `figment` | Configuration layering | [GitHub](https://github.com/figment-rs/figment) |
| `camino` | UTF-8 path types | [GitHub](https://github.com/camino-rs/camino) |

## Prior Art

- [Ripgrep](https://github.com/BurntSushi/ripgrep) — Fast search with ignore support
- [Tantivy](https://github.com/quickwit-oss/tantivy) — Full-text search engine
- [sqlite-fts5](https://www.sqlite.org/fts5.html) — SQLite full-text search
- [Model Context Protocol](https://modelcontextprotocol.io/) — Standard for AI tool interfaces

## License

MIT License — see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

### Code Style

- Follow Rust standard style (`rustfmt`)
- Add tests for new functionality
- Update documentation for user-facing changes
