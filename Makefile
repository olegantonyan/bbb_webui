TARGET=arm-unknown-linux-gnueabihf
EXECUTABLE=$(shell grep name Cargo.toml | grep -Po "\".*\"" | cut -d '"' -f 2)
DIST_PATH=$(EXECUTABLE)-$(TARGET)

dist:
	cargo build --target=$(TARGET) --release
	mkdir $(DIST_PATH)
	cp Rocket.toml $(DIST_PATH)
	cp -R src/assets $(DIST_PATH)
	cp -R src/templates $(DIST_PATH)
	cp target/$(TARGET)/release/$(EXECUTABLE) $(DIST_PATH)
	tar -cjvf $(DIST_PATH).tar.bz2 $(DIST_PATH)
	rm -rf $(DIST_PATH)
