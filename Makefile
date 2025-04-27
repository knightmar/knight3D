CC = gcc
CFLAGS = -Wall -O2
LDFLAGS = -lSDL2 -lGL -lGLU
SRC = $(shell find src -name '*.c')
BIN = sdl3d

all: $(BIN)

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(BIN) $(LDFLAGS)

clean:
	rm -f $(BIN)