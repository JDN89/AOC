#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

/*void scannar(char *source) {}*/

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
  printf("%s", buffer);
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
