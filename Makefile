test:
	@echo "Testing library..."
	@rustc -vV
	@cargo test --lib
.PHONY: test
