# -- Make Configuration --

# Default
ARCH = i386
WORKERS = 4

# Grub Variables
GRUB_SRC = iso

# -- CMake Configuration --

# Basic
NAME = kfs.elf
RELEASE_TYPE = Debug
CURRENT_DIR = ${CURDIR}
BUILD_DIR = out
GENERATOR_TYPE = Ninja

# Cmake Linker Variables
LINKER_EXECUTABLE_PATH = /usr/bin/ld.lld
LINKER_SCRIPT = $(CURRENT_DIR)/arch/$(ARCH)/linker.ld
LINKER_FLAGS = -m elf_i386 -z noexecstack -n

# CMake ASM Variables
ASM_SRCS = $(CURRENT_DIR)/arch/$(ARCH)/multiboot.asm $(CURRENT_DIR)/arch/$(ARCH)/boot.asm
ASM_OBJECT_TYPE = elf32
ASM_FLAGS = -f elf32 -noexecstack

# CMake C Variables
CFLAGS =
C_SRCS =
INCLUDES = includes/42_logo.h

# CMake Rust Variables
RUST_TARGET = $(CURRENT_DIR)/$(ARCH)-kfs.json
RUST_SRCS = src/lib.rs

# Recipes
all: $(NAME)

re: fclean all

$(BUILD_DIR): CMakeLists.txt build.rs $(RUST_SRCS) $(C_SRCS) $(INCLUDES) $(ASM_SRCS) $(LINKER_SCRIPT)
	@MAKE_NAME="$(NAME)" MAKE_LINKER_SCRIPT="$(LINKER_SCRIPT)" MAKE_LINKER_FLAGS="$(LINKER_FLAGS)" \
	MAKE_ASM_FLAGS="$(ASM_FLAGS)" MAKE_ASM_SRCS="$(ASM_SRCS)" MAKE_RUST_TARGET="$(RUST_TARGET)" \
	MAKE_ASM_OBJECT_TYPE="$(ASM_OBJECT_TYPE)" MAKE_LINKER_EXECUTABLE_PATH="$(LINKER_EXECUTABLE_PATH)" \
	MAKE_C_SRCS="$(C_SRCS)" MAKE_CFLAGS="$(CFLAGS)" MAKE_INCLUDES="$(INCLUDES)" \
	cmake -G "$(GENERATOR_TYPE)" -DCMAKE_BUILD_TYPE=$(RELEASE_TYPE) -S . -B $(BUILD_DIR)

$(BUILD_DIR)/$(NAME): $(BUILD_DIR)
	@cmake --build $(BUILD_DIR) -- -j $(WORKERS)

$(NAME): $(BUILD_DIR)/$(NAME)
	@cp $(BUILD_DIR)/$(NAME) .

run-iso: $(NAME).iso
	@qemu-system-$(ARCH) -cdrom $(NAME).iso

# Bugged
run: $(NAME)
	@qemu-system-$(ARCH) -kernel $(NAME) -machine type=pc-i440fx-3.1

$(NAME).iso: $(NAME) $(GRUB_SRC)/boot/grub/grub.cfg
	@grub-file --is-x86-multiboot2 $(NAME)
	@cp $(BUILD_DIR)/$(NAME) $(GRUB_SRC)/boot
	@grub-mkrescue --compress=xz -o $(NAME).iso $(GRUB_SRC)

fclean:
	@rm -rf $(NAME)
	@rm -rf $(NAME).iso
	@rm -rf $(GRUB_SRC)/boot/$(NAME)
	@rm -rf $(BUILD_DIR)
	@cargo clean

clean:
	@cmake --build $(BUILD_DIR) -- clean
	@cmake --build $(BUILD_DIR) -- cargo-clean


.PHONY: all re fclean clean run run-iso install_tooling
