package ast

import (
	"testing"

	"github.com/alfredoprograma/fredlang/lexer"
)

func TestParseLiteral(t *testing.T) {
	tokens := []lexer.Token{
		lexer.NewToken(lexer.String, "Hello world", 1),
		lexer.NewToken(lexer.Integer, "10", 1),
		lexer.NewToken(lexer.Float, "15.5", 1),
		lexer.NewToken(lexer.True, "true", 1),
		lexer.NewToken(lexer.False, "false", 1),
		lexer.NewToken(lexer.Null, "null", 1),
	}

	expectedExprs := []Primary{
		{"Hello world"},
		{10},
		{15.5},
		{true},
		{false},
		{nil},
	}

	for i, token := range tokens {
		p := NewParser([]lexer.Token{token})
		primary := p.parseLiteral()
		expectedPrimary := expectedExprs[i]

		if primary != expectedPrimary {
			t.Errorf("mismatching primary expressions. expected %#v, but got %#v", expectedPrimary, primary)
		}
	}
}

func TestParseGroup(t *testing.T) {
	tokens := []lexer.Token{
		lexer.NewToken(lexer.LParen, lexer.LParen.Lexeme(), 1),
		lexer.NewToken(lexer.String, "Example", 1),
		lexer.NewToken(lexer.RParen, lexer.RParen.Lexeme(), 1),
	}

	expectedExpr := Primary{
		Primary{"Example"},
	}

	p := NewParser(tokens)
	group := p.parsePrimary()

	if group != expectedExpr {
		t.Errorf("mismatching group expression. expected %#v, but got %#v", group, expectedExpr)
	}
}
