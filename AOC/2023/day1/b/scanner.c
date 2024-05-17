#include "scanner.h"
#include "string.h"

#include <stdbool.h>
#include <stdio.h>

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

// advance and return consumed character
static char advance() {
  scanner.current++;
  return scanner.current[-1];
}

static void advanceBy(int by) { scanner.current += by; }

// move pointer with value start and compare value until end with rest of string
static int checkNumber(int start, int length, const char *rest, int returnValue,
                       int jump) {
  int comp = memcmp(scanner.start + start, rest, length);

  if (comp == 0) {
    advanceBy(jump);
    return returnValue;
  } else {
    // don't advance because we allready advance one char and returned the
    // consume char
    return NO_MATCH;
  }
}

int scanSource() {

  scanner.start = scanner.current;

  if (isAtEnd()) {
    return END_OF_FILE;
  }

  char c = advance();
  printf("char c :: %c \n", c);

  if (c == '\n') {
    return LINE_BREAK;
  }

  if (c == '\0') {
    return END_OF_FILE;
  }

  if (isDigit(c)) {
    printf("isDigit: %c\n", c);
    return c - '0';
  }

  if (isAlpha(c)) {
    switch (scanner.start[0]) {
    case 'o':
      return checkNumber(1, 2, "ne", 1, 2);
    case 't':
      switch (scanner.start[1]) {
      case 'w':
        return checkNumber(2, 1, "o", 2, 2);
      case 'h':
        return checkNumber(2, 3, "ree", 3, 4);
      }
      break;
    case 'f':
      switch (scanner.start[1]) {
      case 'o':
        return checkNumber(2, 2, "ur", 4, 3);
      case 'i':
        return checkNumber(2, 2, "ve", 5, 3);
      }
      break;
    case 's':
      switch (scanner.start[1]) {
      case 'i':
        return checkNumber(2, 1, "x", 6, 2);
      case 'e':
        return checkNumber(2, 3, "ven", 7, 4);
      }
      break;
    case 'e':
      return checkNumber(1, 4, "ight", 8, 4);

    case 'n':
      return checkNumber(1, 3, "ine", 9, 3);
    }
  }

  return NO_MATCH;
}
