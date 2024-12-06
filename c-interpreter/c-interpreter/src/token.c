#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include "../include/token.h"

const char ILLEGAL[] = "ILLEGAL";
const char EOF_IDENT[] = "EOF";

const char IDENT[] = "IDENT";
const char INTEGER[] = "INT";
const char ASSIGN[] = "=";
const char PLUS[] = "+";
const char MINUS[] = "-";
const char BANG[] = "!";
const char ASTERIX[] = "*";
const char SLASH[] = "/";

const char LT[] = "<";
const char GT[] = ">";

const char EQ[] = "==";
const char NQ[] = "!=";

const char COMMA[] = ",";
const char SEMICOLON[] = ";";

const char LPAREN[] = "(";
const char RPAREN[] = ")";
const char LBRACE[] = "{";
const char RBRACE[] = "}";

const char FUNCTION[] = "FUNCTION";
const char LET[] = "LET";
const char TRUE_KEYWORD[] = "TRUE";
const char FALSE_KEYWORD[] = "FALSE";
const char IF_KEYWORD[] = "IF";
const char ELSE_KEYWORD[] = "ELSE";
const char RETURN_KEYWORD[] = "RETURN";

TokenTypeMap keywords[] = {
    {"fn", FUNCTION},
    {"let", LET},
    {"true", TRUE_KEYWORD},
    {"false", FALSE_KEYWORD},
    {"if", IF_KEYWORD},
    {"else", ELSE_KEYWORD},
    {"return", RETURN_KEYWORD},
};

P_TokenType lookupIdent(char *ident) {
    for (int i = 0; i < MAX_SIZE; i++) {
        if (strcmp(keywords[i].key, ident) == 0) {
            return keywords[i].value;
        }
    }
    return IDENT;
}

Token *new_Token(P_TokenType tokenType, char *literal) {
    Token *t = malloc(sizeof(Token));
    
    if (t == NULL) {
        printf("Error: Could not allocate memory.\n");
        return NULL;
    }

    char *temp_literal = strdup(literal);
    if (temp_literal == NULL) {
        free(t);
        return NULL;
    }

    t->type = tokenType;
    t->literal = temp_literal;

    return t;
}

void free_Token(Token *t) {
    free(t->literal);
    free(t);
}
