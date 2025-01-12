package ast

import (
	"goInterpreter/chimera/internal/token"
	"testing"
)

func TestString(t *testing.T) {
	program := &Program{
		Statements: []Statement{
			&LetStatement{
				Token: token.Token{Type: token.LET, Literal: "let"},
				Name: &Identifier{
					Token: token.Token{Type: token.IDENT, Literal: "myVariable"},
					Value: "myVariable",
				},
				Value: &Identifier{
					Token: token.Token{Type: token.IDENT, Literal: "anotherVariable"},
					Value: "anotherVariable",
				},
			},
		},
	}

	if program.String() != "let myVariable = anotherVariable;" {
		t.Errorf("program.String() wrong. got=%q", program.String())
	}
}
