package ast

import (
	"fmt"

	"github.com/alfredoprograma/fredlang/lexer"
)

type Node interface {
	String() string
}

type Primary struct {
	value any
}

func (p Primary) String() string {
	switch p.value.(type) {
	case nil:
		return fmt.Sprint(lexer.Null.Lexeme())
	case Node:
		node := p.value.(Node)
		return fmt.Sprintf("(%s)", node.String())
	default:
		return fmt.Sprintf("%v", p.value)
	}
}

type Unary struct {
	op   lexer.Token
	node Node
}

func (u *Unary) String() string {
	return fmt.Sprintf("(%s%s)", u.op.Lexeme(), u.node.String())
}

type Binary struct {
	left  Node
	op    lexer.Token
	right Node
}

func (b *Binary) String() string {
	return fmt.Sprintf("(%s %s %s)", b.left.String(), b.op.Lexeme(), b.right.String())
}
