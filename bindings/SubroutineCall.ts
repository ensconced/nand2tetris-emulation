// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Expression } from "./Expression";

export type SubroutineCall = { Direct: { subroutine_name: string, arguments: Array<[Expression, number]>, } } | { Method: { this_name: string, method_name: string, arguments: Array<[Expression, number]>, } };