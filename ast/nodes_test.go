package ast

import (
	"testing"

	"github.com/alfredoprograma/fredlang/lexer"
)

func TestEvalBinary(t *testing.T) {
	exprs := []Binary{
		{Primary{10.0}, lexer.NewToken(lexer.Plus, lexer.Plus.Lexeme(), 1), Primary{20.0}},
		{Primary{20.0}, lexer.NewToken(lexer.Minus, lexer.Minus.Lexeme(), 1), Primary{10.0}},
		{Primary{5.0}, lexer.NewToken(lexer.Star, lexer.Star.Lexeme(), 1), Primary{5.0}},
		{Primary{30.0}, lexer.NewToken(lexer.Slash, lexer.Slash.Lexeme(), 1), Primary{5.0}},
		{Primary{100.0}, lexer.NewToken(lexer.Greater, lexer.Greater.Lexeme(), 1), Primary{50.0}},
		{Primary{12.0}, lexer.NewToken(lexer.GreaterEq, lexer.GreaterEq.Lexeme(), 1), Primary{12.0}},
		{Primary{100.0}, lexer.NewToken(lexer.Less, lexer.Less.Lexeme(), 1), Primary{50.0}},
		{Primary{12.0}, lexer.NewToken(lexer.LessEq, lexer.LessEq.Lexeme(), 1), Primary{12.0}},
		{Primary{true}, lexer.NewToken(lexer.DoubleEq, lexer.DoubleEq.Lexeme(), 1), Primary{true}},
		{Primary{"DIFFERENT"}, lexer.NewToken(lexer.BangEq, lexer.BangEq.Lexeme(), 1), Primary{"different"}},
	}

	expectedValues := []any{30.0, 10.0, 25.0, 6.0, true, true, false, true, true, true}

	for i, expr := range exprs {
		value, _ := expr.Eval()
		expectedValue := expectedValues[i]

		if value != expectedValue {
			t.Errorf("#%v mismatching evaluation result for binary expression. expected %t, but got %t", i, expectedValue, value)
		}
	}

}

func TestEvalUnary(t *testing.T) {
	exprs := []Unary{
		{lexer.NewToken(lexer.Minus, lexer.Minus.Lexeme(), 1), Primary{10}},
		{lexer.NewToken(lexer.Minus, lexer.Minus.Lexeme(), 1), Primary{0.15}},
		{lexer.NewToken(lexer.Bang, lexer.Bang.Lexeme(), 1), Primary{false}},
		{lexer.NewToken(lexer.Bang, lexer.Bang.Lexeme(), 1), Unary{
			lexer.NewToken(lexer.Bang, lexer.Bang.Lexeme(), 1),
			Primary{false},
		}},
	}

	expectedValues := []any{-10, -0.15, true, false}

	for i, expr := range exprs {
		value, _ := expr.Eval()
		expectedValue := expectedValues[i]

		if value != expectedValue {
			t.Errorf("mismatching evaluation result for unary expression. expected %t, but got %t", expectedValue, value)
		}
	}
}

func TestEvalLiteralPrimary(t *testing.T) {
	exprs := []Primary{
		{"Hello world"},
		{10},
		{99.9},
		{true},
		{false},
		{nil},
	}
	expectedValues := []any{"Hello world", 10, 99.9, true, false, nil}

	for i, expr := range exprs {
		value, _ := expr.Eval()
		expectedValue := expectedValues[i]

		if value != expectedValue {
			t.Errorf("mismatching evaluation result for literal primary expression. expected %t, but got %t", expectedValue, value)
		}
	}
}
