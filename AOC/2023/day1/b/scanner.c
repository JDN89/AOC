#include "scanner.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  const char *start;
  const char *current;
  int line;
} Scanner;

Scanner scanner;

// source is a pointer to the current position in the source file
void initScanner(const char *source) {
  scanner.start = source;
  scanner.current = source;
  scanner.line = 1;
}

static bool isAtEnd() { return *scanner.current == '\0'; }

static bool isAlpha(char c) {
  return ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'));
}

static bool isDigit(char c) { return (c >= '0' && c <= '9'); }

// advance and retrieve consumed character
static char advance() {
  scanner.current++;
  return scanner.current[-1];
}

int scanSource() {

  if (isAtEnd()) {
    return 0;
  }

  scanner.start = scanner.current;
  char c = advance();
  if (isAlpha(c)) {
    printf("alpha: %c \n", *scanner.current);
  }
  if (isDigit(c)) {
    int digit = atoi(scanner.current);
    printf("digit: %d \n", digit);
  }

  return 0;
}
