IMAGE_FILE=disk.img
EFI_FILE=target/x86_64-unknown-uefi/release/efi-hello.efi
IMAGE_DIR=disk
DEVENV_DIR=devenv

.PHONY: noop
noop:
	echo

.PHONY: build
build:
	cargo build --release

.PHONY: image-dir
image-dir: build
	rm -rf $(IMAGE_DIR)
	mkdir -p $(IMAGE_DIR)/EFI/BOOT
	cp $(EFI_FILE) $(IMAGE_DIR)/EFI/BOOT/BOOTX64.EFI


.PHONY: image-file
image-file: image-dir
	rm -f $(IMAGE_FILE)
	truncate -s 200MiB $(IMAGE_FILE)
	mkfs.fat -n 'MIKAN OS' -s 2 -f 2 -R 32 -F 32 $(IMAGE_FILE)

	mcopy -i $(IMAGE_FILE) -s disk/* ::

ovmf:
	mkdir ovmf
	wget https://raw.githubusercontent.com/uchan-nos/mikanos-build/c78c13b1b86ec30e57412736683ef7c2c1984ab9/devenv/OVMF_CODE.fd -O ovmf/OVMF_CODE.fd
	wget https://raw.githubusercontent.com/uchan-nos/mikanos-build/c78c13b1b86ec30e57412736683ef7c2c1984ab9/devenv/OVMF_VARS.fd -O ovmf/OVMF_VARS.fd

.PHONY: reset-ovmf
reset-ovmf: ovmf
	cp ovmf/* devenv/

.PHONY: run
run: image-file reset-ovmf
	cp ovmf/* devenv/

	qemu-system-x86_64 \
		-m 1G \
		-drive if=pflash,format=raw,readonly,file=$(DEVENV_DIR)/OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=$(DEVENV_DIR)/OVMF_VARS.fd \
		-drive if=ide,index=0,media=disk,format=raw,file=$(IMAGE_FILE) \
		-device nec-usb-xhci,id=xhci \
		-device usb-mouse -device usb-kbd \
		-monitor stdio
