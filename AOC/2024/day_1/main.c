#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  char *start;
  char *current;
} Scanner;

bool is_digit(char c) { return '0' <= c && c <= '9'; }

void advance(Scanner scanner) { scanner.current++; }
char peek(Scanner scanner) { return scanner.current[0]; }

static void skipWhitespace(Scanner scanner) {
  for (;;) {
    char c = peek(scanner);
    switch (c) {
    case ' ':
    case '\r': // carriage return
    case '\t': // tab
    case '\n': // new line
      advance(scanner);
      break;
    default:
      return;
    }
  }
}

int lex_source(Scanner scanner) {
  skipWhitespace(scanner);
  while (*scanner.current != '\0') {
    while (is_digit(peek(scanner))) {
      advance(scanner);
    }
    char *char_num = scanner.current - scanner.start;

    int num = *char_num - '0';
    return num;

    // convert chars to digits
    //

    // scan and convert next number
  }
}

void init_scanner(char *source) {

  // Initialize the scanner; start = current
  Scanner scanner = {source, source};
  int num = lex_source(scanner);
}

void read_file(char *path) {

  // open the binary file in read mode
  FILE *file = fopen(path, "rb");
  assert(file);
  fseek(file, 0L, SEEK_END);
  size_t file_size = ftell(file);
  rewind(file);

  char *buffer = malloc(sizeof(char) * file_size + 1);
  assert(buffer);

  size_t bytes_read = fread(buffer, sizeof(char), file_size, file);
  buffer[bytes_read] = '\0';
  /*printf("%s", buffer);*/
  init_scanner(buffer);
}

#include <string.h>
int main(int argc, char *argv[]) {

  if (argc == 1) {
    return printf("repl");
  } else if (argc == 2) {
    read_file(argv[1]);
  }
  return 0;
}

void test() {
  char *source = "12";
  init_scanner(source);
}
