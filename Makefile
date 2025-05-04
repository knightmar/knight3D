CC = gcc
CFLAGS = -Wall -g -O0 -I./include
LDFLAGS = -lSDL2 -lGL -lGLU -lGLEW -lm
SRC = $(shell find src -name '*.c')
OBJ = $(SRC:.c=.o)
BIN = sdl3d

# Default target
all: $(BIN)

# Link the object files into the final binary
$(BIN): $(OBJ)
	$(CC) $(OBJ) -o $(BIN) $(LDFLAGS)

# Compile each source file to an object file
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up object files and the binary
clean:
	rm -f $(OBJ) $(BIN)
