# ORE App - Agent Guidelines

## Build Commands
- **Web build**: `npx tailwindcss -i ./input.css -o ./public/tailwind.css --minify && dx build`
- **Desktop build**: `dx build --platform desktop`
- **Web serve**: `dx serve`
- **Desktop serve**: `dx serve --platform desktop`
- **Format**: `cargo fmt`
- **Single test**: `cargo test test_name` (replace `test_name` with specific test)

## Project Structure
- Dioxus 0.6+ frontend framework (Rust-based React-like)
- Multi-platform: `web` (default) and `desktop` features
- Solana blockchain integration (mining app)
- Modules: `components/`, `hooks/`, `pages/`, `gateway/`, `utils/`

## Devnet Configuration
- **Network**: Currently configured for Solana Devnet
- **RPC/WSS**: Using `https://api.devnet.solana.com` and `wss://api.devnet.solana.com`
- **Program**: Points to local `../ore/api` for devnet program ID (`CWyZM2fBZUJzxM7rgw2Q8sXAsBfj3LNQVMmWWiox1r3F`)
- **Mint**: Using custom devnet mint address from `ore_api::consts::MINT_ADDRESS`
- **To switch back to mainnet**: Update `Cargo.toml` ore-api to use git repo and change RPC URLs in `src/gateway/mod.rs`

## Code Style
- **Naming**: `snake_case` for files/functions, `PascalCase` for components/types
- **Imports**: Group stdlib → external crates → local modules, alphabetized
- **Components**: Dioxus `rsx!` macro, return `Element`
- **Hooks**: Prefix with `use_`, return signals or resources
- **Error handling**: Use custom `GatewayError` enum with `From` impls, log errors with `log::error!`
- **Feature flags**: Wrap platform-specific code with `#[cfg(feature = "web")]` or `#[cfg(feature = "desktop")]`
- **Types**: Explicit types preferred, leverage Solana SDK types (`Pubkey`, `Signature`)
