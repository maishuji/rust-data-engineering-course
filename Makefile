
# Bind Makefile to cargo
# Usage : 
# 	make build SUBFOLDER=path/to/subproject
# -------------------------------------------------------
DEFAULT_SUBFOLDER = week1
SUBFOLDER ?= $(DEFAULT_SUBFOLDER)

build:
	cd $(SUBFOLDER) && cargo build

run:
	cd $(SUBFOLDER) && cargo run

format:
	cd $(SUBFOLDER) && cargo format

lint:
	cd $(SUBFOLDER) && cargo lint

deploy: cd $(SUBFOLDER) && cargo
