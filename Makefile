help:
	@echo "Available targets:" && \
	 echo "  setup     - Download and set up libtorch" && \
	 echo "  clean     - Remove build artifacts and libtorch" && \
	 echo "  run-ham   - Run the ham email test" && \
	 echo "  run-spam  - Run the spam email test" && \
	 echo "  help      - Show this help message"

.PHONY: help setup clean

# Set 'help' as the default target
.DEFAULT_GOAL := help

setup:
	mkdir -p libtorch; \
		echo "Downloading libtorch..."; \
		curl -L https://download.pytorch.org/libtorch/cpu/libtorch-macos-arm64-2.6.0.zip -o libtorch.zip; \
		unzip libtorch.zip -d .; \
		rm libtorch.zip; \
		export LIBTORCH=$(PWD)/libtorch; \
		echo "LIBTORCH set to $$LIBTORCH"; \

clean:
	rm -rf libtorch libtorch.zip
	cargo clean

run-ham:
	cargo run -- ./src/assets/test_email_ham.txt

run-spam:
	cargo run -- ./src/assets/test_email_spam.txt

