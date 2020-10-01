setup:
		@echo "===================================================================="
		@echo "Setting up..."
		@echo "===================================================================="
		@echo "Add Windows targets"
		@echo "===================================================================="
		rustup target add x86_64-pc-windows-gnu
		@echo "===================================================================="
		@echo "Add Linux targets"
		@echo "===================================================================="
		rustup target add x86_64-unknown-linux-musl

build:
		@echo "===================================================================="
		@echo "Building..."
		@echo "===================================================================="
		cargo build

build-linux:
		@echo "===================================================================="
		@echo "Building linux release..."
		@echo "===================================================================="
		cargo build --release --target=x86_64-pc-windows-gnu

build-windows:
		@echo "===================================================================="
		@echo "Building windows release..."
		@echo "===================================================================="
		cargo build --release --target=x86_64-unknown-linux-musl