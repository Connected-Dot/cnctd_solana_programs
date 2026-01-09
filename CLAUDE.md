# CLAUDE.md - cnctd_solana_programs

> Reference for the Solana on-chain programs.

## Purpose

Collection of Anchor-based Solana smart contracts for on-chain program logic in the cnctd.studio ecosystem.

## Programs

### cnctd_studio_program
**Program ID:** `CSPd6eauKNBXfrQnKmqrHKEjt6xtW7mgzmfV2XPfiy5i`

On-chain program for cnctd.studio handling NFT releases, token transfers, and payment processing.

## Project Structure

```
cnctd_solana_programs/
├── Anchor.toml          # Anchor configuration
├── Cargo.toml           # Workspace
├── programs/
│   └── cnctd_studio_program/
│       └── src/lib.rs   # Program entry point
├── tests/               # TypeScript tests
└── package.json         # Test dependencies
```

## Development

```bash
# Build programs
anchor build

# Test
anchor test

# Deploy
anchor deploy
```

## Dependencies

- Anchor v0.31.0
- anchor-spl (SPL token CPI)

## Ecosystem Role

- **Part of**: cnctd.studio ecosystem
- **Interacts with**: cnctd_solana client library

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
