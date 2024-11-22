package lexer

type TokenKind string

const (
	LParen     TokenKind = "LParen"
	RParen     TokenKind = "RParen"
	LBrace     TokenKind = "LBrace"
	RBrace     TokenKind = "RBrace"
	Comma      TokenKind = "Comma"
	Dot        TokenKind = "Dot"
	Minus      TokenKind = "Minus"
	Plus       TokenKind = "Plus"
	Semicolon  TokenKind = "Semicolon"
	Slash      TokenKind = "Slash"
	Star       TokenKind = "Star"
	Bang       TokenKind = "Bang"
	BangEq     TokenKind = "BangEq"
	Eq         TokenKind = "Eq"
	DoubleEq   TokenKind = "DoubleEq"
	Greater    TokenKind = "Greater"
	GreaterEq  TokenKind = "GreaterEq"
	Less       TokenKind = "Less"
	LessEq     TokenKind = "LessEq"
	String     TokenKind = "String"
	Integer    TokenKind = "Integer"
	Float      TokenKind = "Float"
	Identifier TokenKind = "Identifier"
	And        TokenKind = "And"
	Class      TokenKind = "Class"
	Else       TokenKind = "Else"
	False      TokenKind = "False"
	Function   TokenKind = "Function"
	For        TokenKind = "For"
	If         TokenKind = "If"
	Null       TokenKind = "Null"
	Or         TokenKind = "Or"
	Print      TokenKind = "Print"
	Return     TokenKind = "Return"
	Super      TokenKind = "Super"
	This       TokenKind = "This"
	True       TokenKind = "True"
	Var        TokenKind = "Var"
	While      TokenKind = "While"
	EOF        TokenKind = "EOF"
)

var lexemeToKindMap = map[string]TokenKind{
	"(":        LParen,
	")":        RParen,
	"{":        LBrace,
	"}":        RBrace,
	",":        Comma,
	".":        Dot,
	"-":        Minus,
	"+":        Plus,
	";":        Semicolon,
	"/":        Slash,
	"*":        Star,
	"!":        Bang,
	"!=":       BangEq,
	"=":        Eq,
	"==":       DoubleEq,
	">":        Greater,
	">=":       GreaterEq,
	"<":        Less,
	"<=":       LessEq,
	"and":      And,
	"class":    Class,
	"else":     Else,
	"false":    False,
	"function": Function,
	"for":      For,
	"if":       If,
	"null":     Null,
	"or":       Or,
	"print":    Print,
	"return":   Return,
	"super":    Super,
	"this":     This,
	"true":     True,
	"var":      Var,
	"while":    While,
}

// Derives the inverted map from existing `lexemeToKindMap`
var kindToLexemeMap = func() map[TokenKind]string {
	m := map[TokenKind]string{}

	for lexeme, kind := range lexemeToKindMap {
		m[kind] = lexeme
	}

	return m
}()

func tokenKindFromKeyword(keyword string) (TokenKind, bool) {
	kind, ok := lexemeToKindMap[keyword]

	if !ok {
		return "", false
	}

	return kind, true
}

func (t TokenKind) Lexeme() string {
	lexeme, ok := kindToLexemeMap[t]

	if !ok {
		return ""
	}

	return lexeme
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

func NewToken(kind TokenKind, lexeme string, line int) Token {
	return Token{
		kind,
		lexeme,
		line,
	}
}

func (t *Token) Kind() TokenKind {
	return t.kind
}

func (t *Token) Lexeme() string {
	return t.lexeme
}
