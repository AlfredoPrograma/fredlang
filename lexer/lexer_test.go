package lexer

import "testing"

func TestScanTokens(t *testing.T) {
	source := "()   {   },.-+;/*!!= ==\n   =<<=>>=  \n\"Hello world\"\n1234 12.25 .9\nor and function myVar"
	lexer := New(source)
	expectedTokens := []Token{
		NewToken(LParen, LParen.Lexeme(), 1),
		NewToken(RParen, RParen.Lexeme(), 1),
		NewToken(LBrace, LBrace.Lexeme(), 1),
		NewToken(RBrace, RBrace.Lexeme(), 1),
		NewToken(Comma, Comma.Lexeme(), 1),
		NewToken(Dot, Dot.Lexeme(), 1),
		NewToken(Minus, Minus.Lexeme(), 1),
		NewToken(Plus, Plus.Lexeme(), 1),
		NewToken(Semicolon, Semicolon.Lexeme(), 1),
		NewToken(Slash, Slash.Lexeme(), 1),
		NewToken(Star, Star.Lexeme(), 1),
		NewToken(Bang, Bang.Lexeme(), 1),
		NewToken(BangEq, BangEq.Lexeme(), 1),
		NewToken(DoubleEq, DoubleEq.Lexeme(), 1),
		NewToken(Eq, Eq.Lexeme(), 2),
		NewToken(Less, Less.Lexeme(), 2),
		NewToken(LessEq, LessEq.Lexeme(), 2),
		NewToken(Greater, Greater.Lexeme(), 2),
		NewToken(GreaterEq, GreaterEq.Lexeme(), 2),
		NewToken(String, "Hello world", 3),
		NewToken(Number, "1234", 4),
		NewToken(Number, "12.25", 4),
		NewToken(Dot, Dot.Lexeme(), 4),
		NewToken(Number, "9", 4),
		NewToken(Or, Or.Lexeme(), 5),
		NewToken(And, And.Lexeme(), 5),
		NewToken(Function, Function.Lexeme(), 5),
		NewToken(Identifier, "myVar", 5),
		NewToken(EOF, "", 6),
	}

	tokens, _ := lexer.ScanTokens()

	if len(tokens) != len(expectedTokens) {
		t.Errorf("resulting and expected tokens have different lengths. expected %d but got %d", len(expectedTokens), len(tokens))
	}

	for i, token := range tokens {
		expected := expectedTokens[i]

		if token != expected {
			t.Errorf("mismatching tokens. expected %#v, but got %#v", expected, token)
		}
	}
}
