#include "scanner.h"
#include "string.h"

#include <stdbool.h>
#include <stdio.h>

// TODO: place define and function used in main in a header file!

#define NO_MATCH 69;
#define LINE_BREAK 666;
#define END_OF_FILE 999;

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
static int checkNumber(int start, int length, const char *rest, int number) {
  printf("comp: start - %d, length - %d, rest - %s, number - %d \n \n", start,
         length, rest, number);
  int comp = memcmp(scanner.start + start, rest, length);

  if (comp == 0) {
    advanceBy(length);
    return number;
  } else {
    advance();
    printf("no mathc in check: %c \n", *scanner.current);
    return NO_MATCH;
  }
}

int scanSource() {

  scanner.start = scanner.current;

  if (isAtEnd()) {
    return END_OF_FILE;
  }

  char c = advance();

  if (c == '\n') {
    return LINE_BREAK;
  }

  if (c == '\0') {
    return END_OF_FILE;
  }

  if (isDigit(c)) {
    return c - '0';
  }

  if (isAlpha(c)) {
    printf("%c: charcter is this \n", c);
    switch (scanner.start[0]) {
    case 'o':
      return checkNumber(1, 2, "ne", 1);
    case 't':
      // FIX: advance is not correct in this case -> level deeper so advance is
      // plus one!!
      // add an extra argument int advance in case of two  advance is two and
      // three advance is four!

      switch (scanner.start[1]) {
      case 'w':
        return checkNumber(2, 1, "o", 2);
      case 'h':
        return checkNumber(2, 3, "ree", 3);
      }
      break;
    }
  }

  printf("no hit \n");
  return NO_MATCH;
}
