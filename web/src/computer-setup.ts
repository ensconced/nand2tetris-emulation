import { make_computer as makeComputer } from "../../web-emulator/pkg/web_emulator";

import data from "../debug-output.json";
import { CompilerResult } from "../bindings/CompilerResult";

const compilerResult = data as CompilerResult;
const {
  assembly_result: { instructions },
} = compilerResult;

const rom = new Uint16Array(instructions);
export default makeComputer(rom);
