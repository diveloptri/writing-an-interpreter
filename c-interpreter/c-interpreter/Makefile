SRC_DIR = ./src
BIN_DIR = ./bin
TEST_DIR = ./tests
BUILD_DIR = ./build

CC = gcc
CFLAGS = -Wall -Wextra -I$(TEST_DIR)

SRC = $(wildcard $(SRC_DIR)/*.c)
TEST_SRC = $(wildcard $(TEST_DIR)/*.c)

SRC_OBJ = $(SRC:$(SRC_DIR/%.c=$(BUILD_DIR)/%.o))
TEST_OBJ = $(filter-out $(BUILD_DIR)/main.o, $(SRC_OBJ)) $(TEST_SRC:$(TEST_DIR)/%.c=$(BUILD_DIR)/%.o)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: $(TEST_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

run:
	$(CC) -g -I include $(SRC_DIR)/*.c -o $(BIN_DIR)/main && $(BIN_DIR)/main

test: $(TEST_OBJ) 
	$(CC) $(TEST_OBJ) -o $(BUILD_DIR)/test
	./$(BUILD_DIR)/test


clean:
	rm -rf $(BUILD_DIR)