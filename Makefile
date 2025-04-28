CC = gcc
CFLAGS = -Wall -O2 -DGLEW_NO_GLU
LDFLAGS = -lSDL2 -lGL -lGLU -lGLEW
SRC = $(shell find src -name '*.c')
BIN = sdl3d

all: $(BIN)

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(BIN) $(LDFLAGS)

clean:
	rm -f $(BIN)