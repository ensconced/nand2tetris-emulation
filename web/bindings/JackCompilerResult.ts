// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { JackCompilerSourceMap } from "./JackCompilerSourceMap";
import type { Token } from "./Token";
import type { TokenKind } from "./TokenKind";

export interface JackCompilerResult { sourcemaps: Record<string, JackCompilerSourceMap>, tokens: Record<string, Array<Token<TokenKind>>>, subroutines: Record<string, Array<CompiledSubroutine>>, }