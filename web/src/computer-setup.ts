import { make_computer as makeComputer } from "../../web-emulator/pkg/web_emulator";

import data from "../debug-output.json";
import { CompilerResult } from "../bindings/CompilerResult";

const compilerResult = data as CompilerResult;
const {
  assembly_result: { instructions },
} = compilerResult;

const rom = new Uint16Array(instructions);
const computer = makeComputer(rom);

// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
(window as any).computer = computer;
export default computer;
