# -- Make Configuration --

# Default
ARCH = i386
WORKERS = 4

# Grub Variables
GRUB_SRC = iso

# -- CMake Configuration --

# Basic
NAME = kfs.elf
RELEASE_TYPE = Release
CURRENT_DIR = ${CURDIR}
BUILD_DIR = build
GENERATOR_TYPE = Ninja

# Cmake Linker Variables
LINKER_EXECUTABLE_PATH = /usr/bin/ld.lld
LINKER_SCRIPT = $(CURRENT_DIR)/arch/$(ARCH)/linker.ld
LINKER_FLAGS = -m elf_i386 -z noexecstack -n

# CMake ASM Variables
ASM_SRCS = $(wildcard $(CURRENT_DIR)/arch/$(ARCH)/*.asm)
ASM_OBJECT_TYPE = elf32
ASM_FLAGS = -f elf32 -F dwarf -g

# CMake C Variables
CFLAGS =
C_SRCS =
INCLUDES =

# CMake Rust Variables
RUST_TARGET = $(CURRENT_DIR)/$(ARCH)-kfs.json
RUST_SRCS = $(wildcard src/*.rs)

# Recipes
all: $(NAME) $(NAME).iso

re: fclean all

corrosion/CMakeLists.txt:
	@git submodule init
	@git submodule update

$(BUILD_DIR): CMakeLists.txt build.rs $(RUST_SRCS) $(C_SRCS) $(INCLUDES) $(ASM_SRCS) $(LINKER_SCRIPT) corrosion/CMakeLists.txt
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
# -boot d changes boot order for CD/DVD drive first, -display curses is for text mode
	@qemu-system-$(ARCH) -boot d -cdrom $(NAME).iso -display curses

run-debug: $(NAME).iso
# To quit a text mode debugging session: Alt+2, type "q" or "quit" in the qemu console
# With GDB running, "kill" does the trick. There's an "fq" alias for that in the .gdbinit file.
# -s: same as "-gdb tcp::1234", to have a debugging session
# -S: Don't start the CPU, wait for continue call from GDB
	@qemu-system-$(ARCH) -boot d -cdrom $(NAME).iso -s -S -display curses

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

.PHONY: all re fclean clean run run-iso
