import React, { useMemo, useState } from "react";
import {
  make_computer as makeComputer,
  get_formatted_ram,
  tick,
} from "../../web-emulator/pkg/web_emulator";
import TabControls from "./TabControls";

const rom = new Int16Array(32768);
const computer = makeComputer(rom);
tick(computer);
tick(computer);
tick(computer);

const wordDisplayBaseOptions = ["binary", "decimal"];

export default function Computer() {
  const [wordDisplayBaseIdx, setWordDisplayBaseIdx] = useState(0);
  const wordDisplayBase = wordDisplayBaseOptions[wordDisplayBaseIdx];

  const ram = useMemo(() => {
    const ramString = get_formatted_ram(computer.ram, wordDisplayBaseIdx);
    return ramString.split("\n");
  }, [wordDisplayBaseIdx]);

  return (
    <div className="panel-container">
      <TabControls
        items={wordDisplayBaseOptions}
        onChange={(idx) => setWordDisplayBaseIdx(idx)}
        checkedIdx={wordDisplayBaseIdx}
        groupName="ram-display"
      />
      <code style={{ textAlign: "right" }} className="code-panel">
        {ram.map((line) => (
          <span>{`${line}\n`}</span>
        ))}
      </code>
    </div>
  );
}
