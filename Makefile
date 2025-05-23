CC = gcc
CFLAGS = -Wall -g -O0
LDFLAGS = -lSDL2 -lGL -lGLU -lm
SRC = $(shell find src -name '*.c')
BIN = sdl3d

all: $(BIN)

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(BIN) $(LDFLAGS)

clean:
	rm -f $(BIN)