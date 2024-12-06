#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "../include/repl.h"
#include "../include/lexer.h"
#include "../include/constants.h"

const char *PROMPT = ">> ";

void start_repl() {
    while(1){
        char *input_buffer = NULL;
        size_t buffer_size = 0;
        size_t input_length;

        printf("%s", PROMPT);

        input_length = getline(&input_buffer, &buffer_size, stdin);
        if (input_length == -1) {
            if (feof(stdin)) {
                printf("Exit REPL");
                free(input_buffer);
                break;
            } else {
                perror("Error: Failed to read input");
                free(input_buffer);
                continue;
            }
        }

        input_buffer[strcspn(input_buffer, "\n")] = '\0';

        if (strlen(input_buffer) == 0) {
            free(input_buffer);
            continue;
        }

        Lexer *l = new_Lexer(input_buffer);
        if (l == NULL) {
            printf("Error: Could not allocate Lexer");
            free(input_buffer);
            continue;
        }

        if (strcmp(input_buffer, "exit") == 0) {
            break;
        }

        Token *token;
        while ((token = next_Token(l)) != NULL) {
            if (token->type == EOF_IDENT) {
                free_Token(token);
                break;
            }

            printf("{ Type: %s || Literal: %s }\n", token->type, token->literal);
            free_Token(token);
        }

        if (token == NULL) {
            fprintf(stderr, "Error: Token creation failed\n");
        }

        free(input_buffer);
        free_Lexer(l);
    }
}
