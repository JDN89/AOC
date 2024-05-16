#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "scanner.h"

// ONE TWO THREE FOUR FIVE SIX SEVEN EIGHT NINE

#define TEST_INPUT "test_input.txt"
#define ONE_INPUT "one.txt"
#define INPUT "input.txt"
#define BASE 10

typedef struct {
  char num;
  bool isInitialized;
} Num;

long result = 0;

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
    exit(74);
  }
  size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);
  if (bytesRead < fileSize) {
    fprintf(stderr, "Could not read file \"%s\".\n", filename);
    exit(74);
  }
  buffer[bytesRead] = '\0';
  fclose(file);
  return buffer;
}

void process_input(const char *input) {

  const char *ptr = input;
  initScanner(ptr);

  for (;;) {

    int digit = scanSource();
    printf("digit: %d \n", digit);

    if (digit == END_OF_FILE) {
      printf("End of file reached \n");
      break;
    }
  }

  return;
}

int main() {

  clock_t start = clock();

  char *contents = read_from_file(ONE_INPUT);
  process_input(contents);

  free(contents);

  clock_t stop = clock();
  double time_spent = (double)(stop - start) * 1000 / CLOCKS_PER_SEC;
  printf("Time elapsed in ms: %f\n", time_spent);

  return 0;
}
