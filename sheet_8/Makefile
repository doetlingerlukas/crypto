TARGET = decode_natural_number

CXXFLAGS = -O3 -std=c++1z

all: $(TARGET)

clean:
	$(RM) *.o $(TARGET)

run: all
	./$(TARGET) 10010111111110001001010101100010

.PHONY: clean run

$(TARGET): %: %.o
	$(CXX) $(CXXFLAGS) $^ -o $@
