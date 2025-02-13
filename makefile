# --- Top-Level Variables ---
VARIATIONAL_PDF = quantum_physics_variational_overview
VARIATIONAL_TEX_FILE = ./variational.tex
BUILD_DIR = build
DOCS_DIR = docs

# Path to the official Solana sbf.mk
SBF.MK ?= $(HOME)/.local/share/solana/install/active_release/bin/sdk/sbf/c/sbf.mk

# Variables used by sbf.mk (for the on-chain program build)
SRC_DIR = ./src
OUT_DIR = ./build

# Let the user override the include path at build time or via environment
# The default is a typical location after installing Solana
INC_DIRS ?= $(HOME)/.local/share/solana/install/active_release/bin/sdk/sbf/c/inc

# --- Targets ---
.PHONY: all pdf variational tools clean

# Build everything: PDF, on-chain program, and Rust tool
all: pdf variational tools

# Build the LaTeX PDF
pdf: $(BUILD_DIR) $(DOCS_DIR) $(VARIATIONAL_PDF).pdf

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(DOCS_DIR):
	mkdir -p $(DOCS_DIR)

$(VARIATIONAL_PDF).pdf: $(VARIATIONAL_TEX_FILE)
	@echo "Building PDF from $(VARIATIONAL_TEX_FILE)..."
	pdflatex -interaction=nonstopmode \
		-output-directory=$(BUILD_DIR) \
		-jobname=$(VARIATIONAL_PDF) \
		$(VARIATIONAL_TEX_FILE)
	mv $(BUILD_DIR)/$(VARIATIONAL_PDF).pdf $(DOCS_DIR)

# Build the Solana on-chain shared object for the variational program using sbf.mk.
# We pass the necessary variables so that sbf.mk knows where to find sources and where to put outputs.
variational:
	@echo "Building on-chain program (variational)..."
	$(MAKE) -f $(SBF.MK) variational SRC_DIR=$(SRC_DIR) OUT_DIR=$(OUT_DIR) INC_DIRS=$(INC_DIRS)

# Build the Rust transaction sender (in tools/transaction_sender)
tools:
	@echo "Building Rust transaction sender..."
	cargo build --manifest-path=tools/transaction_sender/Cargo.toml

# Clean up all generated files
clean:
	@echo "Cleaning up..."
	rm -rf $(BUILD_DIR)
	rm -rf $(DOCS_DIR)
	rm -f $(VARIATIONAL_PDF).pdf
	# $(MAKE) -f $(SBF.MK) clean SRC_DIR=$(SRC_DIR) OUT_DIR=$(OUT_DIR) INC_DIRS=$(INC_DIRS)
	cargo clean --manifest-path=tools/transaction_sender/Cargo.toml
