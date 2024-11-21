package lexer

type TokenKind string

const (
	LParen    TokenKind = "LParen"
	RParen    TokenKind = "RParen"
	LBrace    TokenKind = "LBrace"
	RBrace    TokenKind = "RBrace"
	Comma     TokenKind = "Comma"
	Dot       TokenKind = "Dot"
	Minus     TokenKind = "Minus"
	Plus      TokenKind = "Plus"
	Semicolon TokenKind = "Semicolon"
	Slash     TokenKind = "Slash"
	Star      TokenKind = "Star"
	Bang      TokenKind = "Bang"
	BangEq    TokenKind = "BangEq"
	Eq        TokenKind = "Eq"
	DoubleEq  TokenKind = "DoubleEq"
	Greater   TokenKind = "Greater"
	GreaterEq TokenKind = "GreaterEq"
	Less      TokenKind = "Less"
	LessEq    TokenKind = "LessEq"
	String    TokenKind = "String"
	Integer   TokenKind = "Integer"
	Float     TokenKind = "Float"
)

func (t TokenKind) Lexeme() string {
	switch t {
	case LParen:
		return "("
	case RParen:
		return ")"
	case LBrace:
		return "{"
	case RBrace:
		return "}"
	case Comma:
		return ","
	case Dot:
		return "."
	case Minus:
		return "-"
	case Plus:
		return "+"
	case Semicolon:
		return ";"
	case Slash:
		return "/"
	case Star:
		return "*"
	case Bang:
		return "!"
	case BangEq:
		return "!="
	case Eq:
		return "="
	case DoubleEq:
		return "=="
	case Greater:
		return ">"
	case GreaterEq:
		return ">="
	case Less:
		return "<"
	case LessEq:
		return "<="
	default:
		return ""
	}
}

func (t TokenKind) Rune() rune {
	lexeme := t.Lexeme()

	if len(lexeme) != 1 {
		return 0
	}

	return rune(lexeme[0])
}

type Token struct {
	kind   TokenKind
	lexeme string
	line   int
}

func newToken(kind TokenKind, lexeme string, line int) Token {
	return Token{
		kind,
		lexeme,
		line,
	}
}
