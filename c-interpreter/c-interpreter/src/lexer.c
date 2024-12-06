#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include "../include/lexer.h"
#include "../include/token.h"
#include "../include/constants.h"

Lexer *new_Lexer(const char *input) {
    char *input_cp = strdup(input);
    Lexer *l = malloc(sizeof(Lexer));

    if (l == NULL) {
        printf("Error: Could not allocate memory.\n");
        return NULL;
    }

    l->input = input_cp;

    l->position = 0;
    l->readPosition = 0;

    readChar(l);
    return l;
}

void free_Lexer(Lexer *l) {
    free(l->input);
    free(l);
}

void readChar(Lexer *l) {
    size_t len = strlen(l->input);
    if (l->position >= len) {
        l->character = 0;
    } else {
        l->character = l->input[l->readPosition];
    }
    l->position = l->readPosition;
    l->readPosition++;
}

char peekChar(Lexer *l) {
    size_t len = strlen(l->input);
    if (l->readPosition >= len) {
        return 0;
    }else {
        return l->input[l->readPosition];
    }
}

void skipWhiteSpace(Lexer *l) {
    while (l->character == ' ' || l->character == '\t' || l->character == '\n' || l->character == '\r') {
        readChar(l);
    }
}

char* slice_string(char *str, int start, int end) {
    int length = end - start + 1;
    char *slice = (char*)malloc(length);
    if (slice == NULL) {
        return NULL;
    }
    strncpy(slice, str + start, length -1);
    slice[length - 1] = '\0';
    return slice;
}

char* char_to_literal(char c) {
    char *literal = (char*)malloc(2 * sizeof(char));
    if (literal == NULL) {
        fprintf(stderr, "Error on memory allocation!\n");
        return NULL;
    }
    snprintf(literal, 2, "%c", c);
    return literal;
}

char* char_concant(char a, char b) {
    char *literal = (char*)malloc(3 * sizeof(char));
    if (literal == NULL) {
        fprintf(stderr, "Error on memory allocation!\n");
        return NULL;
    }
    snprintf(literal, 3, "%c%c", a, b);
    return literal;
}

int isDigit(char c);
int isDigit(char c) {
    if (c >= '0' && c <= '9') {
        return 1;
    }
    return 0;
}

int isLetter(char c) {
    if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_') {
        return 1;
    }
    return 0;
}

char* readNumber(Lexer *l) {
    int position = l->position;
    while (isDigit(l->character) == 1) {
        readChar(l);
    }
    char* number_str = (char*)malloc(sizeof(l->position - position));
    number_str = strdup(slice_string(l->input, position, l->position));
    return number_str;
} 

char* readIdentifier(Lexer *l) {
    size_t position = l->position;
    while(isLetter(l->character) == 1) {
        readChar(l);
    }
    char* ident_str = (char*)malloc(sizeof(l->position - position));
    char* slice_str = slice_string(l->input, position, l->position);
    ident_str = strdup(slice_str);
    return ident_str;
}

Token *next_Token(Lexer *l) {
    Token *t = NULL;

    skipWhiteSpace(l);

    switch (l->character)
    {
    case '=':
        if (peekChar(l) == '=') {
            char ch = l->character;
            readChar(l);
            t = new_Token(EQ, char_concant(ch, l->character));
        } else {
            t = new_Token(ASSIGN, char_to_literal(l->character));
        }
        break;
    case '+':
        t = new_Token(PLUS, char_to_literal(l->character));
        break;
    case '-':
        t = new_Token(MINUS, char_to_literal(l->character));
        break;
    case '!':
        if (peekChar(l) == '=') {
            char ch = l->character;
            readChar(l);
            t = new_Token(NQ, char_concant(ch, l->character));
        } else {
            t = new_Token(BANG, char_to_literal(l->character));
        }
        break;
    case '*':
        t = new_Token(ASTERIX, char_to_literal(l->character));
        break;
    case '/':
        t = new_Token(SLASH, char_to_literal(l->character));
        break;
    case '<':
        t = new_Token(LT, char_to_literal(l->character));
        break;
    case '>':
        t = new_Token(GT, char_to_literal(l->character));
        break;
    case ',':
        t = new_Token(COMMA, char_to_literal(l->character));
        break;
    case ';':
        t = new_Token(SEMICOLON, char_to_literal(l->character));
        break;
    case '(':
        t = new_Token(LPAREN, char_to_literal(l->character));
        break;
    case ')':
        t = new_Token(RPAREN, char_to_literal(l->character));
        break;
    case '{':
        t = new_Token(LBRACE, char_to_literal(l->character));
        break;
    case '}':
        t = new_Token(RBRACE, char_to_literal(l->character));
        break;
    case '\0':
        t = new_Token(EOF_IDENT, "");
        break;
    
    default:
        if (isLetter(l->character) == 1) {
            char *literal = readIdentifier(l);
            t = new_Token(lookupIdent(literal), literal);
            return t;
        } else if (isDigit(l->character) == 1){
            char *literal = readNumber(l);
            t = new_Token(INTEGER, literal);
            return t;
        } else {
            t = new_Token(ILLEGAL, char_to_literal(l->character));
            break;
        }
    }

    if (t == NULL) {
        fprintf(stderr, "Critical: Unable to create token at position %zu\n", l->position);
        return NULL;
    }

    readChar(l);
    return t;
}

