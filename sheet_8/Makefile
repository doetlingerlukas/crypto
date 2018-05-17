CC = g++
LD = g++

OBJ = decode_natural_number.o
TARGET = dnn

CFLAGS = -O3 -std=c++1y

# Rules
all: $(TARGET)

%.o: %.cpp
	$(CC) $(CFLAGS) -c $^ -o $@

clean:
	rm -f *.o $(TARGET)
	
run: clean all
		./dnn 10010111111110001001010101100010

.PHONY: clean run

# Dependencies
$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $^ -o $@
