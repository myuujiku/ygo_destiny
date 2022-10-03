.PHONY: clean install install-local uninstall

target/release/ygod_gtk : src
	cargo build --release

install : target/release/ygod_gtk
	@echo Installing to \'/usr\'...
	@cp target/release/ygod_gtk /usr/bin/com.myujiku.ygod
	@cp resources/com.myujiku.ygod.desktop /usr/share/applications/

install-local : target/release/ygod_gtk
	@echo Installing to \'~/.local\'...
	@cp target/release/ygod_gtk ~/.local/bin/com.myujiku.ygod
	@cp resources/com.myujiku.ygod.desktop ~/.local/share/applications/

uninstall :
	@if [[ -w /usr ]]; then\
		echo Removing files in \'/usr\'...;\
		rm -f /usr/bin/com.myujiku.ygod;\
		rm -f /usr/share/application/com.myujiku.ygod.desktop;\
	else\
		echo If you used \'sudo make install\' you should run this with sudo as well.;\
	fi

	@echo Removing files in \'/home/$$(logname)/.local\'...
	@rm -f /home/$$(logname)/.local/bin/com.myujiku.ygod
	@rm -f /home/$$(logname)/.local/share/application/com.myujiku.ygod.desktop

clean :
	@echo Cleaning build files...
	@cargo clean
	@echo Done.
