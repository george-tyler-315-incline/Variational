# Output names and directories
VARIATIONAL_PDF = quantum_physics_variational_overview
VARIATIONAL_TEX_FILE = ./variational.tex
BUILD_DIR = build
DOCS_DIR = docs

# Default target should be first
.DEFAULT_GOAL := all

# Main build target
all: $(BUILD_DIR) $(DOCS_DIR) $(VARIATIONAL_PDF).pdf

# Create build directory
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Create docs directory
$(DOCS_DIR):
	mkdir -p $(DOCS_DIR)

# Rule to create PDF
$(VARIATIONAL_PDF).pdf: $(VARIATIONAL_TEX_FILE)
	pdflatex -interaction=nonstopmode \
		-output-directory=$(BUILD_DIR) \
		-jobname=$(VARIATIONAL_PDF) \
		$(VARIATIONAL_TEX_FILE)
	mv $(BUILD_DIR)/$(VARIATIONAL_PDF).pdf $(DOCS_DIR)

# Clean generated files
clean:
	rm -rf $(BUILD_DIR)
	rm -rf $(DOCS_DIR)
	rm -f $(VARIATIONAL_PDF).pdf

.PHONY: all clean