CC = gcc
CFLAGS = -Wall -Wextra -std=c99
LDFLAGS = -lSDL2 -lGL -lGLEW -lm

SRC = src/main.c src/renderer.c src/shader.c src/obj_files/obj_parser.c src/objects/triangle.c src/objects/shape.c
OBJ = $(SRC:.c=.o)
BIN = bin/opengl_app

all: $(BIN)

$(BIN): $(OBJ)
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -I/usr/include/SDL2 -I/usr/include/GL -c -o $@ $<

clean:
	rm -f $(OBJ) $(BIN)

.PHONY: all clean
