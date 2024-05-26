# Satoshis Palace BullVsBear

## Build Project:
```
make build-mainnet
```

## Testing
```
make test
```

## Code Coverage Setup

### Steps to Set Up Code Coverage for Rust Smart Contracts
0. **If the following steps don't work you may need to install `openssl`**
   ```sh
   sudo apt-get update
   sudo apt-get install pkg-config libssl-dev
   ```

1. **Install `cargo-tarpaulin`**:
   ```sh
   cargo install cargo-tarpaulin
   ```

2. **Run the Tests**:
   ```sh
   make test
   ```

3. **Generate the Coverage Report**:
   ```sh
   make coverage
   ```

4. **Open the Coverage Report**:
   ```sh
   make open-coverage
   ```

## Generate Schema
```
make schema
```