# Architecture & Relayer Guide

## Directory Structure
- `apps/web`: Next.js 14 Web Application
- `packages/contract-client`: TypeScript SDK wrapping Soroban XDR invocations
- `services/relay`: Go daemon cbridge listening for Soroban RPC events and interacting with Anchors

---

## Configuration (.env)
```env
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
SOROBAN_RPC_URL="https://soroban-testnet.stellar.org:443"
NEXT_PUBLIC_ESCROW_CONTRACT_ID="CCCSLE7UN2FRLB2HQWEUEXM4365NDYH3QSC6J5TILQWBSTTIDKFWXX2Y"
RELAY_SIGNER_SECRET="S..."
TARGET_ANCHOR_DOMAIN="testanchor.stellar.org"
PORT=8080
```

