UUID = ibus-hold@geosp.github.io
ZIP = $(UUID).shell-extension.zip

.PHONY: pack install test clean

pack:
	gnome-extensions pack --force --extra-source=engines.js $(UUID)

install: pack
	gnome-extensions install --force $(ZIP)

test:
	gjs -m test/engines-test.js

clean:
	rm -f $(ZIP)
