#ifndef TOKEN_H
#define TOKEN_H
 
typedef const char *P_TokenType;
 
typedef struct {
    P_TokenType type;
    char *literal;
} Token;
 
Token *new_Token(P_TokenType tokenType, char *literal);
void free_Token(Token *token);
 
#define MAX_SIZE 7
typedef struct {
    char key[MAX_SIZE];
    P_TokenType value;
} TokenTypeMap;
 
P_TokenType lookupIdent(char *ident);

#endif