package ast

import (
	"errors"
	"fmt"

	"github.com/alfredoprograma/fredlang/lexer"
)

type Node interface {
	String() string
	Eval() (any, error)
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

func (p Primary) Eval() (any, error) {
	switch p.value.(type) {
	case Node:
		node := p.value.(Node)
		return node.Eval()
	default:
		// Strings, integers, floats, booleans and nil primitive values are
		// directly extracted
		return p.value, nil
	}
}

type Unary struct {
	op   lexer.Token
	node Node
}

func (u Unary) String() string {
	return fmt.Sprintf("(%s%s)", u.op.Lexeme(), u.node.String())
}

func (u Unary) Eval() (any, error) {
	nodeValue, err := u.node.Eval()
	op := u.op.Kind()

	if err != nil {
		return nil, err
	}

	switch value := nodeValue.(type) {
	case int, float64:
		return u.evalNumberOperation(value, op)
	case bool:
		return u.evalBooleanOperation(value, op)
	default:
		return nil, errors.New("cannot evaluate unary expression")
	}
}

func (u Unary) evalNumberOperation(value any, op lexer.TokenKind) (any, error) {
	if op != lexer.Minus {
		return nil, errors.New("cannot evaluate numeric sign invertion on unary expression")
	}

	switch number := value.(type) {
	case int:
		return -number, nil
	case float64:
		return -number, nil
	default:
		panic("something went really wrong if u are here")
	}
}

func (u Unary) evalBooleanOperation(value bool, op lexer.TokenKind) (bool, error) {
	if op != lexer.Bang {
		return false, errors.New("cannot evaluate boolean negation on unary expression")
	}

	return !value, nil
}

func operatorsAsNumbers(left, right any) (float64, float64, error) {
	leftNumber, ok := left.(float64)

	if !ok {
		return 0, 0, errors.New("left operator is not a number")
	}

	rightNumber, ok := right.(float64)

	if !ok {
		return 0, 0, errors.New("right operator is not a number")
	}

	return leftNumber, rightNumber, nil
}

var arithmeticOperations = map[lexer.TokenKind]func(left, right float64) float64{
	lexer.Plus: func(left, right float64) float64 {
		return left + right
	},
	lexer.Minus: func(left, right float64) float64 {
		return left - right
	},
	lexer.Star: func(left, right float64) float64 {
		return left * right
	},
	lexer.Slash: func(left, right float64) float64 {
		return left / right
	},
}

var comparisonOperations = map[lexer.TokenKind]func(left, right float64) bool{
	lexer.Greater: func(left, right float64) bool {
		return left > right
	},
	lexer.GreaterEq: func(left, right float64) bool {
		return left >= right
	},
	lexer.Less: func(left, right float64) bool {
		return left < right
	},
	lexer.LessEq: func(left, right float64) bool {
		return left <= right
	},
}

var equalityOperations = map[lexer.TokenKind]func(left, right any) bool{
	lexer.DoubleEq: func(left, right any) bool {
		return left == right
	},
	lexer.BangEq: func(left, right any) bool {
		return left != right
	},
}

type Binary struct {
	left  Node
	op    lexer.Token
	right Node
}

func (b Binary) String() string {
	return fmt.Sprintf("(%s %s %s)", b.left.String(), b.op.Lexeme(), b.right.String())
}

func (b Binary) Eval() (any, error) {
	left, err := b.left.Eval()

	if err != nil {
		return nil, err
	}

	right, err := b.right.Eval()

	if err != nil {
		return nil, err
	}

	op := b.op.Kind()

	switch op {
	case lexer.Plus, lexer.Minus, lexer.Star, lexer.Slash:
		leftNumber, rightNumber, err := operatorsAsNumbers(left, right)

		if err != nil {
			return nil, err
		}

		arithmeticFunc := arithmeticOperations[op]

		return arithmeticFunc(leftNumber, rightNumber), nil
	case lexer.Greater, lexer.GreaterEq, lexer.Less, lexer.LessEq:
		leftNumber, rightNumber, err := operatorsAsNumbers(left, right)

		if err != nil {
			return nil, err
		}

		comparisonFunc := comparisonOperations[op]

		return comparisonFunc(leftNumber, rightNumber), nil
	case lexer.DoubleEq, lexer.BangEq:
		equalityFunc := equalityOperations[op]

		return equalityFunc(left, right), nil
	default:
		panic("binary operation not implemented")
	}

}
