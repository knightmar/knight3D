CC = gcc
# Add -g for debug symbols and -fsanitize=address for AddressSanitizer
CFLAGS = -Wall -Wextra -std=c99 -g -fsanitize=address 
# Add ASan flag to linker flags as well
LDFLAGS = -lSDL2 -lGL -lGLEW -lm -fsanitize=address 

# Note: Removed src/objects/triangle.c as it seems unused now? Re-check this.
# Let's assume triangle.c is NOT needed based on current rendering path.
SRC = src/main.c src/renderer.c src/shader.c src/obj_files/obj_parser.c src/objects/shape.c
OBJ = $(SRC:.c=.o)
BIN = bin/opengl_app

all: $(BIN)

$(BIN): $(OBJ)
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -I/usr/include/SDL2 -I/usr/include/GL -c -o $@ $< # CFLAGS already includes ASan flags

clean:
	rm -f $(OBJ) $(BIN)

.PHONY: all clean
