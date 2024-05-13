#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define TEST_INPUT "test_input.txt"
#define INPUT "input.txt"
#define BASE 10

// TODO: add better error handling

typedef struct {
  char num;
  bool isInitialized;
} Num;

long result = 0;

char *read_from_file(char *filename) {
  FILE *file = fopen(filename, "r");
  if (file == NULL) {
    perror("Error opening file");
    exit(EXIT_FAILURE);
  }
  fseek(file, 0, SEEK_END);
  int length = ftell(file);
  fseek(file, 0, SEEK_SET);

  char *contents =
      malloc(sizeof(char) * length + 1); // Allocate memory for the contents
  if (contents == NULL) {
    perror("Memory allocation failed");
    exit(EXIT_FAILURE);
  }

  fread(contents, 1, length, file);
  contents[length] = '\0';
  fclose(file);
  return contents;
}

void process_input(const char *input) {

  const char *ptr = input;

  Num first = {.isInitialized = 0};
  Num second = {.isInitialized = 0};

  char concated_chars[2];
  char *output;

  while (*ptr != '\0') {
    if (*ptr == '\n') {

      concated_chars[0] = first.num;
      concated_chars[1] = second.num;

      long temp = strtol(concated_chars, &output, BASE);
      result += temp;

      first.isInitialized = 0;
      first.num = '\0';
      second.num = '\0';
      temp = 0;
    }
    if (isdigit(*ptr)) {
      if (first.isInitialized == 0) {
        first.num = *ptr;
        second.num = *ptr;

        first.isInitialized = 1;
      } else {
        second.num = *ptr;
      }
    }
    ptr++;
  }
}

int main() {

  char *contents = read_from_file(INPUT);
  process_input(contents);

  free(contents);

  printf("result: %ld \n", result);

  return 0;
}
