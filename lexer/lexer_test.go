package lexer

import "testing"

func TestScanTokens(t *testing.T) {
	source := "(){},.-+;/*!!====<<=>>="
	lexer := New(source)
	expectedTokens := []Token{
		newToken(LParen, LParen.Lexeme(), 1),
		newToken(RParen, RParen.Lexeme(), 1),
		newToken(LBrace, LBrace.Lexeme(), 1),
		newToken(RBrace, RBrace.Lexeme(), 1),
		newToken(Comma, Comma.Lexeme(), 1),
		newToken(Dot, Dot.Lexeme(), 1),
		newToken(Minus, Minus.Lexeme(), 1),
		newToken(Plus, Plus.Lexeme(), 1),
		newToken(Semicolon, Semicolon.Lexeme(), 1),
		newToken(Slash, Slash.Lexeme(), 1),
		newToken(Star, Star.Lexeme(), 1),
		newToken(Bang, Bang.Lexeme(), 1),
		newToken(BangEq, BangEq.Lexeme(), 1),
		newToken(DoubleEq, DoubleEq.Lexeme(), 1),
		newToken(Eq, Eq.Lexeme(), 1),
		newToken(Less, Less.Lexeme(), 1),
		newToken(LessEq, LessEq.Lexeme(), 1),
		newToken(Greater, Greater.Lexeme(), 1),
		newToken(GreaterEq, GreaterEq.Lexeme(), 1),
	}

	tokens, _ := lexer.ScanTokens()

	if len(tokens) != len(expectedTokens) {
		t.Errorf("resulting and expected tokens have different lengths. expected %d but got %d", len(expectedTokens), len(source))
	}

	for i, token := range tokens {
		expected := expectedTokens[i]

		if token != expected {
			t.Errorf("mismatching tokens. expected %#v, but got %#v", expected, token)
		}
	}
}
