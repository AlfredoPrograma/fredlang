package lexer

import (
	"errors"
	"strings"
	"unicode"
)

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

		if unicode.IsSpace(ch) {
			l.consumeSpaces()
			continue
		}

		if ch == '"' {
			lexeme, err := l.parseString()

			if err != nil {
				l.errors = append(l.errors, err.Error())
				continue
			}

			token = NewToken(String, lexeme, l.line)
			l.tokens = append(l.tokens, token)
			continue
		}

		if unicode.IsNumber(ch) {
			lexeme := l.parseNumber()
			token = NewToken(Number, lexeme, l.line)
			l.tokens = append(l.tokens, token)
			continue
		}

		if unicode.IsLetter(ch) {
			lexeme, kind := l.parseKeywordOrIdentifier()
			token = NewToken(kind, lexeme, l.line)
			l.tokens = append(l.tokens, token)
			continue
		}

		switch ch {
		case LParen.Rune():
			token = NewToken(LParen, LParen.Lexeme(), l.line)
		case RParen.Rune():
			token = NewToken(RParen, RParen.Lexeme(), l.line)
		case LBrace.Rune():
			token = NewToken(LBrace, LBrace.Lexeme(), l.line)
		case RBrace.Rune():
			token = NewToken(RBrace, RBrace.Lexeme(), l.line)
		case Comma.Rune():
			token = NewToken(Comma, Comma.Lexeme(), l.line)
		case Dot.Rune():
			token = NewToken(Dot, Dot.Lexeme(), l.line)
		case Minus.Rune():
			token = NewToken(Minus, Minus.Lexeme(), l.line)
		case Plus.Rune():
			token = NewToken(Plus, Plus.Lexeme(), l.line)
		case Semicolon.Rune():
			token = NewToken(Semicolon, Semicolon.Lexeme(), l.line)
		case Slash.Rune():
			token = NewToken(Slash, Slash.Lexeme(), l.line)
		case Star.Rune():
			token = NewToken(Star, Star.Lexeme(), l.line)
		case Bang.Rune():
			if l.match(Eq.Rune()) {
				token = NewToken(BangEq, BangEq.Lexeme(), l.line)
			} else {
				token = NewToken(Bang, Bang.Lexeme(), l.line)
			}
		case Eq.Rune():
			if l.match(Eq.Rune()) {
				token = NewToken(DoubleEq, DoubleEq.Lexeme(), l.line)
			} else {
				token = NewToken(Eq, Eq.Lexeme(), l.line)
			}
		case Greater.Rune():
			if l.match(Eq.Rune()) {
				token = NewToken(GreaterEq, GreaterEq.Lexeme(), l.line)
			} else {
				token = NewToken(Greater, Greater.Lexeme(), l.line)
			}
		case Less.Rune():
			if l.match(Eq.Rune()) {
				token = NewToken(LessEq, LessEq.Lexeme(), l.line)
			} else {
				token = NewToken(Less, Less.Lexeme(), l.line)
			}
		default:
			l.errors = append(l.errors, "Unexpected token")
			continue
		}
		l.tokens = append(l.tokens, token)
	}

	l.increaseLine()
	eof := NewToken(EOF, "", l.line)
	l.tokens = append(l.tokens, eof)

	return l.tokens, l.errors
}

func (l *Lexer) advance() rune {
	ch := l.source[l.current]
	l.start = l.current
	l.current++

	return ch
}

func (l *Lexer) lookahead() rune {
	if l.isEnd() {
		return 0
	}

	return l.source[l.current]
}

func (l *Lexer) lookaheadBy(skip int) rune {
	nextIdx := l.start + skip

	if nextIdx >= len(l.source) {
		return 0
	}

	return l.source[nextIdx]
}

func (l *Lexer) peek() rune {
	return l.source[l.start]
}

func (l *Lexer) match(target rune) bool {
	if l.lookahead() == target {
		l.advance()
		return true
	}

	return false
}

func (l *Lexer) isEnd() bool {
	return l.current >= len(l.source)
}

func (l *Lexer) increaseLine() {
	l.line++
}

func (l *Lexer) consumeSpaces() {
	if l.peek() == '\n' {
		l.increaseLine()
	}

	for !l.isEnd() {
		ch := l.lookahead()

		if !unicode.IsSpace(ch) {
			break
		}

		if ch == '\n' {
			l.increaseLine()
		}

		l.advance()
	}
}

func (l *Lexer) parseString() (string, error) {
	var lexeme strings.Builder

	for !l.isEnd() {
		ch := l.advance()

		if ch == '"' {
			return lexeme.String(), nil
		}

		lexeme.WriteRune(ch)
	}

	return "", errors.New("unterminated string")
}

func (l *Lexer) parseNumber() string {
	var lexeme strings.Builder
	lexeme.WriteRune(l.peek())
	isFloat := false

	for !l.isEnd() {
		ch := l.lookahead()

		if ch == '.' && !isFloat {
			chAfterFloat := l.lookaheadBy(2)

			if !unicode.IsNumber(chAfterFloat) {
				break
			}

			isFloat = true
			lexeme.WriteRune('.')
			l.advance()
			continue
		}

		if !unicode.IsNumber(ch) {
			break
		}

		lexeme.WriteRune(ch)
		l.advance()
	}

	return lexeme.String()
}

func (l *Lexer) parseKeywordOrIdentifier() (string, TokenKind) {
	var sb strings.Builder
	sb.WriteRune(l.peek())

	for !l.isEnd() {
		ch := l.lookahead()

		if !unicode.IsLetter(ch) && !unicode.IsNumber(ch) && ch != '_' {
			break
		}

		sb.WriteRune(ch)
		l.advance()
	}

	lexeme := sb.String()

	kind, found := tokenKindFromKeyword(lexeme)

	if found {
		return lexeme, kind
	}

	return lexeme, Identifier
}
