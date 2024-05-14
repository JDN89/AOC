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
    return 69;
  }

  scanner.start = scanner.current;
  char c = advance();
  // TODO: call and use tRIE data structure to retrieve the number, alphaTrie()
  // -> checkNumber(int start, int length,const char *rest, int return type )
  if (isAlpha(c)) {
    printf("alpha: %c \n", c);
  }
  // TODO: if is line break -> return 69 and to nothing check in main if we
  // return 69 in case of 69 do nothing
  if (isDigit(c)) {
    int digit = c - '0';
    printf("digit: %d \n", digit);
  }
  if (c == '\n') {
    printf("\n\n");
  }
  return 69;
}
