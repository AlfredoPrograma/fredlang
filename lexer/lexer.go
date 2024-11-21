package lexer

type Lexer struct {
	source  []rune
	tokens  []Token
	errors  []string // TODO: should be custom errors i think
	start   int
	current int
	line    int
}

func New(source string) Lexer {
	return Lexer{
		source:  []rune(source),
		tokens:  []Token{},
		errors:  []string{},
		start:   0,
		current: 0,
		line:    1,
	}
}

func (l *Lexer) ScanTokens() ([]Token, []string) {
	for !l.isEnd() {
		var token Token
		ch := l.advance()

		switch ch {
		case LParen.Rune():
			token = newToken(LParen, LParen.Lexeme(), l.line)
		case RParen.Rune():
			token = newToken(RParen, RParen.Lexeme(), l.line)
		case LBrace.Rune():
			token = newToken(LBrace, LBrace.Lexeme(), l.line)
		case RBrace.Rune():
			token = newToken(RBrace, RBrace.Lexeme(), l.line)
		case Comma.Rune():
			token = newToken(Comma, Comma.Lexeme(), l.line)
		case Dot.Rune():
			token = newToken(Dot, Dot.Lexeme(), l.line)
		case Minus.Rune():
			token = newToken(Minus, Minus.Lexeme(), l.line)
		case Plus.Rune():
			token = newToken(Plus, Plus.Lexeme(), l.line)
		case Semicolon.Rune():
			token = newToken(Semicolon, Semicolon.Lexeme(), l.line)
		case Slash.Rune():
			token = newToken(Slash, Slash.Lexeme(), l.line)
		case Star.Rune():
			token = newToken(Star, Star.Lexeme(), l.line)
		case Bang.Rune():
			if l.match(Eq.Rune()) {
				token = newToken(BangEq, BangEq.Lexeme(), l.line)
			} else {
				token = newToken(Bang, Bang.Lexeme(), l.line)
			}
		case Eq.Rune():
			if l.match(Eq.Rune()) {
				token = newToken(DoubleEq, DoubleEq.Lexeme(), l.line)
			} else {
				token = newToken(Eq, Eq.Lexeme(), l.line)
			}
		case Greater.Rune():
			if l.match(Eq.Rune()) {
				token = newToken(GreaterEq, GreaterEq.Lexeme(), l.line)
			} else {
				token = newToken(Greater, Greater.Lexeme(), l.line)
			}
		case Less.Rune():
			if l.match(Eq.Rune()) {
				token = newToken(LessEq, LessEq.Lexeme(), l.line)
			} else {
				token = newToken(Less, Less.Lexeme(), l.line)
			}

		default:
			l.errors = append(l.errors, "Unexpected token")
			continue
		}
		l.tokens = append(l.tokens, token)
	}

	return l.tokens, l.errors
}

func (l *Lexer) advance() rune {
	ch := l.source[l.current]
	l.start = l.current
	l.current++

	return ch
}

func (l *Lexer) peek() rune {
	if l.isEnd() {
		return 0
	}

	return l.source[l.current]
}

func (l *Lexer) match(target rune) bool {
	if l.peek() == target {
		l.advance()
		return true
	}

	return false
}

func (l *Lexer) isEnd() bool {
	return l.current >= len(l.source)
}
