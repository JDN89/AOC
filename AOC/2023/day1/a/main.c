#include <stdio.h>
#include <stdlib.h>

#define TEST_INPUT "test_input.txt"

int main() {
  FILE *file = fopen(TEST_INPUT, "r");

  if (file == NULL) {
    printf("Error opening file. \n");
    return 1;
  }

  fseek(file, 0, SEEK_END);
  int length = ftell(file);
  fseek(file, 0, SEEK_SET);

  char *content = malloc(length + 1); // Allocate memory for the contents
  if (content == NULL) {
    printf("Memory allocation error.\n");
    return 1;
  }

  fread(content, 1, length, file);

  content[length] = '\0';

  fclose(file);

  char *ptr = content;
  while (*ptr != '\0') {
    printf("Character: %c, Address: %p\n", *ptr, (void *)ptr);
    ptr++;
  }

  // printf("%s\n", content);

  free(content);

  return 0;
}
