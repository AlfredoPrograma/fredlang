package ast

import (
	"testing"

	"github.com/alfredoprograma/fredlang/lexer"
)

func TestParseFactor(t *testing.T) {
	tokens := []lexer.Token{
		lexer.NewToken(lexer.Integer, "5", 1),
		lexer.NewToken(lexer.Star, lexer.Star.Lexeme(), 1),
		lexer.NewToken(lexer.Integer, "12", 1),
	}

	expectedFactor := Binary{
		left:  Primary{5},
		op:    lexer.NewToken(lexer.Star, lexer.Star.Lexeme(), 1),
		right: Primary{12},
	}

	expectedStringification := "(5 * 12)"

	p := NewParser(tokens)
	factor := p.parseFactor()

	if factor != expectedFactor {
		t.Errorf("mismatching factor expression. expected %#v, but got %#v", expectedFactor, factor)
	}

	if factor.String() != expectedStringification {
		t.Errorf("mismatching factor stringification. expected %s, but got %s", expectedStringification, factor.String())
	}

}

func TestParseUnary(t *testing.T) {
	tokens := []lexer.Token{
		lexer.NewToken(lexer.Minus, lexer.Minus.Lexeme(), 1),
		lexer.NewToken(lexer.Integer, "10", 1),
	}

	expectedUnary := Unary{
		op:   lexer.NewToken(lexer.Minus, lexer.Minus.Lexeme(), 1),
		node: Primary{10},
	}
	expectedStringification := "(-10)"

	p := NewParser(tokens)
	unary := p.parseUnary()

	if unary != expectedUnary {
		t.Errorf("mismatching unary expression. expected %#v, but got %#v", expectedUnary, unary)
	}

	if unary.String() != expectedStringification {
		t.Errorf("mismatching unary stringification. expected %s, but got %s", expectedStringification, unary.String())
	}
}

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
