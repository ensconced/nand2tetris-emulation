// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ASTNode } from "./ASTNode";
import type { BinaryOperator } from "./BinaryOperator";
import type { PrimitiveTermVariant } from "./PrimitiveTermVariant";
import type { SubroutineCall } from "./SubroutineCall";
import type { UnaryOperator } from "./UnaryOperator";

export type Expression = { Parenthesized: ASTNode<Expression> } | { PrimitiveTerm: PrimitiveTermVariant } | { Binary: { operator: BinaryOperator, lhs: ASTNode<Expression>, rhs: ASTNode<Expression>, } } | { Unary: { operator: UnaryOperator, operand: ASTNode<Expression>, } } | { Variable: string } | { SubroutineCall: ASTNode<SubroutineCall> } | { ArrayAccess: { var_name: string, index: ASTNode<Expression>, } };