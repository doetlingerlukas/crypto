TARGET = shift_chiffre

CXXFLAGS = -O3 -std=c++1z

all: $(TARGET)

clean:
	$(RM) *.o $(TARGET)

run: all
	./$(TARGET) UJGUGNNUUGCUJGNNUDAVJGUGCUJQTG

.PHONY: clean run

$(TARGET): %: %.o
	$(CXX) $(CXXFLAGS) $^ -o $@
