package ast

import (
	"fmt"
	"strconv"

	"github.com/alfredoprograma/fredlang/lexer"
)

type Parser struct {
	tokens  []lexer.Token
	current int
	start   int
}

func NewParser(tokens []lexer.Token) Parser {
	return Parser{
		tokens:  tokens,
		current: 0,
		start:   0,
	}
}

func (p *Parser) Parse() Node {
	return p.parseUnary()
}

func (p *Parser) parseUnary() Node {
	if p.match(lexer.Minus, lexer.Bang) {
		op := p.peek()
		p.advance()
		node := p.parseUnary()

		return Unary{op, node}
	}

	return p.parsePrimary()
}

func (p *Parser) parsePrimary() Node {
	if p.match(lexer.LParen) {
		p.advance()
		value := p.Parse() // Top level parse expression

		if !p.match(lexer.RParen) {
			panic("unterminated group expression")
		}

		p.advance()
		return Primary{value}
	}

	if p.match(
		lexer.String,
		lexer.Integer,
		lexer.Float,
		lexer.True,
		lexer.False,
		lexer.Null,
	) {
		return p.parseLiteral()
	}

	return nil
}

func (p *Parser) parseLiteral() Node {
	var value any
	token := p.peek()
	lexeme := token.Lexeme()
	kind := token.Kind()

	switch kind {
	case lexer.String:
		value = lexeme
	case lexer.Integer:
		integer, err := strconv.Atoi(lexeme)

		if err != nil {
			panic(fmt.Sprintf("cannot parse lexeme from given token as integer: %#v", token))
		}

		value = integer
	case lexer.Float:
		float, err := strconv.ParseFloat(lexeme, 32)

		if err != nil {
			panic(fmt.Sprintf("cannot parse lexeme from given token as float: %#v", token))
		}

		value = float
	case lexer.True, lexer.False:
		boolean, err := strconv.ParseBool(lexeme)

		if err != nil {
			panic(fmt.Sprintf("cannot parse lexeme from given token as boolean: %#v", token))
		}

		value = boolean
	case lexer.Null:
		value = nil
	default:
		panic(fmt.Sprintf("cannot parse primary expression from given token: %#v", token))
	}

	p.advance()
	return Primary{value}
}

func (p *Parser) advance() lexer.Token {
	token := p.tokens[p.current]
	p.start = p.current
	p.current++

	return token
}

func (p *Parser) match(targets ...lexer.TokenKind) bool {
	for _, targetKind := range targets {
		token := p.peek()

		if token.Kind() == targetKind {
			return true
		}
	}

	return false
}

func (p *Parser) peek() lexer.Token {
	return p.tokens[p.current]
}
