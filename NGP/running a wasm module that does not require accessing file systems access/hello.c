/*
 * Copyright 2011 The Emscripten Authors.  All rights reserved.
 * Emscripten is available under two separate licenses, the MIT license and the
 * University of Illinois/NCSA Open Source License.  Both these licenses can be
 * found in the LICENSE file.
 */

#include <stdio.h>
#include <string.h>

int main() {
  int x = 0;

  while (x <= 10)
  {
    printf("hello, world! : %d\n\r", x);    
    x++;
  }
  
  return 0;
}
