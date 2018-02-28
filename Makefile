TARGET=arm-unknown-linux-gnueabihf
EXECUTABLE=$(shell grep name Cargo.toml | grep -Po "\".*\"" | cut -d '"' -f 2)
DIST_PATH=$(EXECUTABLE)-$(TARGET)
DEPLOY_TO=debian@beaglebone
BUILD_CONFIG=release

dist:
	cargo build --target=$(TARGET) --$(BUILD_CONFIG)
	rm -rf $(DIST_PATH)
	mkdir $(DIST_PATH)
	cp Rocket.toml $(DIST_PATH)
	cp -R src/assets $(DIST_PATH)
	cp -R src/templates $(DIST_PATH)
	cp target/$(TARGET)/$(BUILD_CONFIG)/$(EXECUTABLE) $(DIST_PATH)
	tar -cjvf $(DIST_PATH).tar.bz2 $(DIST_PATH)
	#rm -rf $(DIST_PATH)

deploy: dist
	rsync -avI $(DIST_PATH) $(DEPLOY_TO):/tmp
	ssh $(DEPLOY_TO) "cd /tmp/$(DIST_PATH) && ROCKET_ENV=production ./$(EXECUTABLE)"
