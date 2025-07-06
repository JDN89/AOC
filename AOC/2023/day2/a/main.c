#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "scanner.h"

// look into scanset

// TODO: Parsing fun!
// scan and when you encounter a number peek next char if char: store game
// id. Scan colors: store number and scan the color. If next char is, keep
// counting and add to total. if peek = ; compare total to max amount per color.
// if exceeded keep advancing \n
// if peek is \n and not exceeded -> add game id to result

// ONE TWO THREE FOUR FIVE SIX SEVEN EIGHT NINE

#define TEST_INPUT "test_input.txt"
#define ONE_INPUT "one.txt"
#define INPUT "input.txt"

#define FINAL_TEST "num_as_char_test.txt"
#define BASE 10

typedef struct {
  int num;
  bool isInitialized;
} Num;

int result = 0;

char *read_from_file(char *filename) {
  FILE *file = fopen(filename, "rb");
  if (file == NULL) {
    fprintf(stderr, "Could not open file \"%s\".\n", filename);
    exit(74);
  }
  fseek(file, 0L, SEEK_END);
  size_t fileSize = ftell(file);
  rewind(file);
  char *buffer = (char *)malloc(fileSize + 1);
  if (buffer == NULL) {
    fprintf(stderr, "Not enough memory to read \"%s\".\n", filename);
    fclose(file);
    exit(74);
  }
  size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);
  if (bytesRead < fileSize) {
    fprintf(stderr, "Could not read file \"%s\".\n", filename);
    fclose(file);
    free(buffer);
    exit(74);
  }
  buffer[bytesRead] = '\0';
  fclose(file);
  return buffer;
}

void process_input(const char *input) {

  Num first = {.isInitialized = 0};
  Num second = {.isInitialized = 0};

  // int num[2];

  const char *ptr = input;
  initScanner(ptr);

  for (;;) {

    int digit = scanSource();

    if (digit == LINE_BREAK) {

      result += first.num * 10 + second.num;
      first.isInitialized = 0;
      first.num = 0;
      second.num = 0;
    } else if (digit == NO_MATCH) {
      // printf("skip digit: %d \n", digit);
    }

    else if (digit == END_OF_FILE) {
      printf("The result: %d \n", result);
      break;
    }

    // when mathc fill in first and second if first.num has'nt been initialized
    // yet. Else just fill in second num.
    else {
      if (first.isInitialized == 0) {
        first.num = digit;
        second.num = digit;
        first.isInitialized = 1;
      } else {
        second.num = digit;
      }
    }
  }

  return;
}

int main() {

  clock_t start = clock();

  char *contents = read_from_file(INPUT);
  process_input(contents);

  free(contents);

  clock_t stop = clock();
  double time_spent = (double)(stop - start) * 1000 / CLOCKS_PER_SEC;
  printf("Time elapsed in ms: %f\n", time_spent);

  return 0;
}
