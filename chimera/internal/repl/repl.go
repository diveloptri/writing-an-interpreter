package repl

import (
	"bufio"
	"fmt"
	"goInterpreter/chimera/internal/evaluator"
	"goInterpreter/chimera/internal/lexer"
	"goInterpreter/chimera/internal/object"
	"goInterpreter/chimera/internal/parser"
	"io"
)

const PROMPT = ">> "

const CHIMERA = `
(╯°□°）╯︵ ┻━┻ 
`

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)
	env := object.NewEnvironment()

	for {
		fmt.Fprint(out, PROMPT)
		scanned := scanner.Scan()

		if !scanned {
			return
		}

		line := scanner.Text()
		// Lexer Struct = readPositon = 1 , ch = first character
		l := lexer.New(line)
		p := parser.New(l)

		program := p.ParseProgram()
		if len(p.Errors()) != 0 {
			printParserErrors(out, p.Errors())
			continue
		}

		evaluated := evaluator.Eval(program, env)
		if evaluated != nil {
			io.WriteString(out, evaluated.Inspect())
			io.WriteString(out, "\n")
		}
	}
}

func printParserErrors(out io.Writer, errors []string) {
	io.WriteString(out, CHIMERA)
	io.WriteString(out, "Woops! We ran into some errors here!\n")
	io.WriteString(out, " parser errors:\n")
	for _, msg := range errors {
		io.WriteString(out, "\t"+msg+"\n")
	}
}
