#include <stdio.h>
#include <unistd.h>
#include <string.h>

#include "../include/lexer.h"
#include "../include/token.h"
#include "../include/repl.h"

int main() {
    char *user = getlogin();

    if (user == NULL) {
        perror("getlogin() error");
        return 1;
    }

    printf("Hello %s! This is the Chimera programming language!\n", user);

    start_repl();

    return 0;
}