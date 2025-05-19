CC = clang
CFLAGS = -fPIC -Wall -Wextra -O2
LDFLAGS = -shared

PKG_CONFIG = pkg-config
PIPEWIRE_CFLAGS = $(shell $(PKG_CONFIG) --cflags libpipewire-0.3)
PIPEWIRE_LIBS = $(shell $(PKG_CONFIG) --libs libpipewire-0.3) -lm

OBJ = $(SRC:.c=.o)

pack: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(LDFLAGS) -o $@ $^ $(PIPEWIRE_LIBS)

%.o: %.c
	$(CC) $(CFLAGS) $(PIPEWIRE_CFLAGS) -c $< -o $@

clean:
	rm -f $(wildcard src/*.o) $(wildcard *.so) exec

.PHONY: all clean

run: $(SRC)
	$(CC) -o exec $(SRC) $(PIPEWIRE_CFLAGS) $(PIPEWIRE_LIBS)
	./exec
	rm -f exec