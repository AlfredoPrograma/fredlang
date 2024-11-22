package ast

import (
	"errors"
	"fmt"
	"strconv"

	"github.com/alfredoprograma/fredlang/lexer"
)

type Parser struct {
	tokens  []lexer.Token
	errors  []error
	current int
}

func NewParser(tokens []lexer.Token) Parser {
	return Parser{
		errors:  []error{},
		tokens:  tokens,
		current: 0,
	}
}

func (p *Parser) Parse() (Node, []error) {
	return p.parseEquality(), p.errors
}

func (p *Parser) parseEquality() Node {
	left := p.parseComparison()

	for !p.isEnd() && p.match(lexer.DoubleEq, lexer.BangEq) {
		op := p.peek()
		p.advance()
		right := p.parseComparison()
		left = Binary{left, op, right}
	}

	return left
}

func (p *Parser) parseComparison() Node {
	left := p.parseTerm()

	for !p.isEnd() && p.match(lexer.Greater, lexer.GreaterEq, lexer.Less, lexer.LessEq) {
		op := p.peek()
		p.advance()
		right := p.parseTerm()
		left = Binary{left, op, right}
	}

	return left
}

func (p *Parser) parseTerm() Node {
	left := p.parseFactor()

	for !p.isEnd() && p.match(lexer.Plus, lexer.Minus) {
		op := p.peek()
		p.advance()
		right := p.parseFactor()
		left = Binary{left, op, right}
	}

	return left
}

func (p *Parser) parseFactor() Node {
	left := p.parseUnary()

	for !p.isEnd() && p.match(lexer.Star, lexer.Slash) {
		op := p.peek()
		p.advance()
		right := p.parseUnary()
		left = Binary{left, op, right}
	}

	return left
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
		value, _ := p.Parse() // Top level parse expression

		if !p.match(lexer.RParen) {
			p.registerError("Unterminated group expression")
		}

		p.advance()
		return Primary{value}
	}

	if p.match(
		lexer.String,
		lexer.Number,
		lexer.True,
		lexer.False,
		lexer.Null,
	) {
		return p.parseLiteral()
	}

	p.registerError("Literal expected")
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
	case lexer.Number:
		number, err := strconv.ParseFloat(lexeme, 32)

		if err != nil {
			panic(fmt.Sprintf("cannot parse lexeme from given token as float: %#v", token))
		}

		value = number
	case lexer.True, lexer.False:
		boolean, err := strconv.ParseBool(lexeme)

		if err != nil {
			panic(fmt.Sprintf("cannot parse lexeme from given token as boolean: %#v", token))
		}

		value = boolean
	case lexer.Null:
		value = nil
	default:
		panic("cannot parse literal")
	}

	p.advance()
	return Primary{value}
}

func (p *Parser) advance() lexer.Token {
	token := p.tokens[p.current]
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

func (p *Parser) isEnd() bool {
	return p.current >= len(p.tokens)
}

func (p *Parser) registerError(message string) {
	p.errors = append(p.errors, errors.New(message))
}
