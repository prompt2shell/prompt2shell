LINUX_FLAGS=
TARGET_WINDOWS=i686-pc-windows-gnu

all: linux_release

windows_debug:
	cargo build --target ${TARGET}

linux_debug:
	${LINUX_FLAGS} cargo build
	cp -rv target/debug/prompt2shell ./prompt2shell_debug

linux_release:
	${LINUX_FLAGS} cargo build --release
	cp -rv target/release/prompt2shell ./

lint:
	cargo-fmt
