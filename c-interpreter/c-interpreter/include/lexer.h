#ifndef LEXER_H
#define LEXER_H

#include "token.h"

typedef struct {
    char *input;
    size_t position;
    size_t readPosition;
    char character;
} Lexer;

Lexer *new_Lexer(const char *input);
void free_Lexer(Lexer *lexer);
void readChar(Lexer *lexer);
Token *next_Token(Lexer *l);


#endif