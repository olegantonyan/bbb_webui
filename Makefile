TARGET=arm-unknown-linux-gnueabihf
BUILD_CONFIG=release

EXECUTABLE=$(shell grep name Cargo.toml | grep -Po "\".*\"" | cut -d '"' -f 2)
DIST_PATH=$(EXECUTABLE)-$(TARGET)

tarball:
	cargo build --target=$(TARGET) --$(BUILD_CONFIG)
	rm -rf $(DIST_PATH)
	mkdir $(DIST_PATH)
	cp Rocket.toml $(DIST_PATH)
	cp bbb_webui.service $(DIST_PATH)
	cp -R src/assets $(DIST_PATH)
	cp -R src/templates $(DIST_PATH)
	cp target/$(TARGET)/$(BUILD_CONFIG)/$(EXECUTABLE) $(DIST_PATH)
	tar -cjvf $(DIST_PATH).tar.bz2 $(DIST_PATH)
	rm -rf $(DIST_PATH)
