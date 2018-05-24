TARGET = decode_lempel_ziv

CXXFLAGS = -O3 -std=c++1z

all: $(TARGET)

clean:
	$(RM) *.o $(TARGET)

run: all
	./$(TARGET) 010110010001111

.PHONY: clean run

$(TARGET): %: %.o
	$(CXX) $(CXXFLAGS) $^ -o $@
