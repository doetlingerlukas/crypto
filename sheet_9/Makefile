CC = g++
LD = g++

OBJ = decode_lempel_ziv.o
TARGET = dlz

CFLAGS = -O3 -std=c++1y

# Rules
all: $(TARGET)

%.o: %.cpp
	$(CC) $(CFLAGS) -c $^ -o $@

clean:
	rm -f *.o $(TARGET)
	
run: clean all
		./dlz 010110010001111

.PHONY: clean run

# Dependencies
$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $^ -o $@
