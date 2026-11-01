CXX := g++

SRC_DIR := src
OBJ_DIR := bin
TARGET := app.app

INCLUDE_DIRS := $(shell find $(SRC_DIR) -type d)
CXXFLAGS := -std=c++20 -Wall -Wextra -g $(addprefix -I,$(INCLUDE_DIRS))

LDLIBS := -lraylib -lGL -lm -lpthread -ldl -lrt -lX11

SOURCES := $(shell find $(SRC_DIR) -type f -name '*.cpp')
OBJECTS := $(patsubst $(SRC_DIR)/%.cpp,$(OBJ_DIR)/%.o,$(SOURCES))

.PHONY: all clean re list debug

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CXX) $(OBJECTS) -o $@ $(LDLIBS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ_DIR) $(TARGET)

re: clean all

list:
	@echo "Sources:"
	@$(foreach src,$(SOURCES),echo "  $(src)";)
	@echo ""
	@echo "Objects:"
	@$(foreach obj,$(OBJECTS),echo "  $(obj)";)

debug:
	@echo "Sources = $(SOURCES)"
	@echo "Objects = $(OBJECTS)"
	@echo "Include dirs = $(INCLUDE_DIRS)"