#include "unity.h"
#include <string.h>
#include <stdio.h>
#include "../src/token.c"
#include "../src/lexer.c"

void setUp(void){}
void tearDown(void){
}

const char* input =
	"let five = 5;\n"
	"let ten = 10;\n"

	"let add = fn(x, y) {\n"
	"	x + y;\n"
	"};\n"
	
	"let result = add(five, ten);\n"
	"!-/*5;\n"
	"5 < 10 > 5;\n"

	"if (5 < 10) {\n"
	"	return true;\n"
	"} else {\n"
	"	return false;\n"
	"}\n"
	
	"10 == 10;\n"
	"10 != 9;\n"
;

typedef const char *P_TokenType;
typedef struct {
    P_TokenType expectedType;
    char *expectedLiteral;
} TestStruct;

#define ARRAY_LENGTH 74

TestStruct* create_input_arr_struct(void) {
    TestStruct* tests = malloc(sizeof(TestStruct) * ARRAY_LENGTH);
    if (tests == NULL) {
        return NULL;
    }

    tests[0] = (TestStruct){LET, strdup("let")};
    tests[1] = (TestStruct){IDENT, strdup("five")};
    tests[2] = (TestStruct){ASSIGN, strdup("=")};
    tests[3] = (TestStruct){INTEGER, strdup("5")};
    tests[4] = (TestStruct){SEMICOLON, strdup(";")};
    tests[5] = (TestStruct){LET, strdup("let")};
    tests[6] = (TestStruct){IDENT, strdup("ten")};
    tests[7] = (TestStruct){ASSIGN, strdup("=")};
    tests[8] = (TestStruct){INTEGER, strdup("10")};
    tests[9] = (TestStruct){SEMICOLON, strdup(";")};
    tests[10] = (TestStruct){LET, strdup("let")};
    tests[11] = (TestStruct){IDENT, strdup("add")};
    tests[12] = (TestStruct){ASSIGN, strdup("=")};
    tests[13] = (TestStruct){FUNCTION, strdup("fn")};
    tests[14] = (TestStruct){LPAREN, strdup("(")};
    tests[15] = (TestStruct){IDENT, strdup("x")};
    tests[16] = (TestStruct){COMMA, strdup(",")};
    tests[17] = (TestStruct){IDENT, strdup("y")};
    tests[18] = (TestStruct){RPAREN, strdup(")")};
    tests[19] = (TestStruct){LBRACE, strdup("{")};
    tests[20] = (TestStruct){IDENT, strdup("x")};
    tests[21] = (TestStruct){PLUS, strdup("+")};
    tests[22] = (TestStruct){IDENT, strdup("y")};
    tests[23] = (TestStruct){SEMICOLON, strdup(";")};
    tests[24] = (TestStruct){RBRACE, strdup("}")};
    tests[25] = (TestStruct){SEMICOLON, strdup(";")};
    tests[26] = (TestStruct){LET, strdup("let")};
    tests[27] = (TestStruct){IDENT, strdup("result")};
    tests[28] = (TestStruct){ASSIGN, strdup("=")};
    tests[29] = (TestStruct){IDENT, strdup("add")};
    tests[30] = (TestStruct){LPAREN, strdup("(")};
    tests[31] = (TestStruct){IDENT, strdup("five")};
    tests[32] = (TestStruct){COMMA, strdup(",")};
    tests[33] = (TestStruct){IDENT, strdup("ten")};
    tests[34] = (TestStruct){RPAREN, strdup(")")};
    tests[35] = (TestStruct){SEMICOLON, strdup(";")};
    tests[36] = (TestStruct){BANG, strdup("!")};
    tests[37] = (TestStruct){MINUS, strdup("-")};
    tests[38] = (TestStruct){SLASH, strdup("/")};
    tests[39] = (TestStruct){ASTERIX, strdup("*")};
    tests[40] = (TestStruct){INTEGER, strdup("5")};
    tests[41] = (TestStruct){SEMICOLON, strdup(";")};
    tests[42] = (TestStruct){INTEGER, strdup("5")};
    tests[43] = (TestStruct){LT, strdup("<")};
    tests[44] = (TestStruct){INTEGER, strdup("10")};
    tests[45] = (TestStruct){GT, strdup(">")};
    tests[46] = (TestStruct){INTEGER, strdup("5")};
    tests[47] = (TestStruct){SEMICOLON, strdup(";")};
    tests[48] = (TestStruct){IF_KEYWORD, strdup("if")};
    tests[49] = (TestStruct){LPAREN, strdup("(")};
    tests[50] = (TestStruct){INTEGER, strdup("5")};
    tests[51] = (TestStruct){LT, strdup("<")};
    tests[52] = (TestStruct){INTEGER, strdup("10")};
    tests[53] = (TestStruct){RPAREN, strdup(")")};
    tests[54] = (TestStruct){LBRACE, strdup("{")};
    tests[55] = (TestStruct){RETURN_KEYWORD, strdup("return")};
    tests[56] = (TestStruct){TRUE_KEYWORD, strdup("true")};
    tests[57] = (TestStruct){SEMICOLON, strdup(";")};
    tests[58] = (TestStruct){RBRACE, strdup("}")};
    tests[59] = (TestStruct){ELSE_KEYWORD, strdup("else")};
    tests[60] = (TestStruct){LBRACE, strdup("{")};
    tests[61] = (TestStruct){RETURN_KEYWORD, strdup("return")};
    tests[62] = (TestStruct){FALSE_KEYWORD, strdup("false")};
    tests[63] = (TestStruct){SEMICOLON, strdup(";")};
    tests[64] = (TestStruct){RBRACE, strdup("}")};
    tests[65] = (TestStruct){INTEGER, strdup("10")};
    tests[66] = (TestStruct){EQ, strdup("==")};
    tests[67] = (TestStruct){INTEGER, strdup("10")};
    tests[68] = (TestStruct){SEMICOLON, strdup(";")};
    tests[69] = (TestStruct){INTEGER, strdup("10")};
    tests[70] = (TestStruct){NQ, strdup("!=")};
    tests[71] = (TestStruct){INTEGER, strdup("9")};
    tests[72] = (TestStruct){SEMICOLON, strdup(";")};
    tests[73] = (TestStruct){EOF_IDENT, ""};

    return tests;
}

void test_next_token(void) {
    Lexer *lexer = new_Lexer(input);
    TestStruct *tests = create_input_arr_struct();

    for (int i = 0; i < ARRAY_LENGTH; i++) {
        Token *token = next_Token(lexer);

        char message[256];

        snprintf(message, sizeof(message),
            "tests[%d] - tokentype wrong, expected = %s, got = %s",
            i,
            tests[i].expectedType,
            token->type
        );

        TEST_ASSERT_EQUAL_STRING_MESSAGE(
            tests[i].expectedType,
            token->type,
            message
        );

        snprintf(message, sizeof(message),
            "tests[%d] - literal wrong, expected = %s, got = %s",
            i,
            tests[i].expectedLiteral,
            token->literal
        );

        TEST_ASSERT_EQUAL_STRING_MESSAGE(
            tests[i].expectedLiteral,
            token->literal,
            message
        );
    }

    free_Lexer(lexer);

}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_next_token);
    return UNITY_END();
}