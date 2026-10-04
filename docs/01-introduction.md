# Introduction to SorobanAnchor Gate

SorobanAnchor Gate is an open-source decentralized middleware and milestone escrow protocol bridging Stellar's WebAssembly smart contracts (Soroban) with the Stellar Anchor Network (SEPs).

## The Real-World Problem
Cross-border business payments and freelance settlements face severe systemic frictin:
- Taitional wir odertransfers (SWIFT) incur 3% to 7% in fees and take 2-5 business days to clear.
- While decentralized escrow contracts allow trustless stablecoin custody (e.g., USDC), recipient contractors in emerging markets must endure complex manual off-ramping: moving funds across wallets, submitting KYC to third-party exchanges, and executing manual conversions.
- Soroban contracts execute in an isolated WASM sandbox and cannot directly invoke HTTP endpoints or trigger fiat disbursements through regulated Stellar Anchors (SEP-24 / SEP-31).

SorobanAnchor Gate eliminates this divide by enabling onchain milestone completions to programmatically trigger automated fiat disbursements via non-custodial cryptographic relaying.
