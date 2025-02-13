# Quantum Physics Variational Methods: Work in Progress

## Description
This repository is still a work in progress, and should be finished before Christmas 2025. The current documentation can be found here: [Variational Method Overview](https://george-tyler-315-incline.github.io/Variational/).

## Prerequisites

### 1. Solana Toolchain
Install the Solana toolchain (which includes the SBF SDK, clang, ld.lld, etc.).
You can install a specific version (e.g., `v1.18.4`) with:
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/solana-install-init.sh)"
solana-install update
```

Verify the installation:
```bash
solana --version
```

### 2. LaTeX Installation
#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install texlive-full
sudo apt install latexmk
```


### 3. Rust and Cargo
Install Rust and Cargo using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify the installation:
```bash
rustc --version
cargo --version
```

## Building the Project

### Clone the Repository
```bash
git clone https://github.com/george-tyler-315-incline/Variational.git
cd variational
```

### Build Options

1. Build everything (PDF, on-chain program, and tools):
```bash
make all
```

2. Build individual components:
   - PDF documentation only:
   ```bash
   make pdf
   ```
   - Solana on-chain program only:
   ```bash
   make variational
   ```
   - Rust transaction sender tool only:
   ```bash
   make tools
   ```

### Clean Build Files
To clean all generated files and build artifacts:
```bash
make clean
```

## Build Output Locations
- PDF documentation: `docs/quantum_physics_variational_overview.pdf`
- Solana program: `build/variational.so`
- Transaction sender: `tools/transaction_sender/target/debug/transaction_sender`

## Deploying and Running the Program Locally

1. Start the Solana Test Validator
```bash
solana-test-validator --reset
```

2. Deploy the Solana Program and Capture Program ID
```bash
__program_id=$(solana program deploy <repo>/build/variational.so | awk '/Program Id:/ { print $3 }')
```

3. Stream Transaction Logs for Deployed Program
```bash
solana logs "$__program_id"
```

4. Send Transaction to Variational Program
```bash
<repo>/tools/transaction_sender/target/debug/transaction-sender
```