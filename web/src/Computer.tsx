import React, { useState } from "react";
import TabControls from "./TabControls";
import {
  get_ram_word,
  Ram,
  WordDisplayBase,
} from "../../web-emulator/pkg/web_emulator";

const wordDisplayBaseOptions = ["binary", "decimal", "binary blocks"];

const ramDepth = 32768;

const addresses = Array(ramDepth)
  .fill(0)
  .map((_val, idx) => idx);

interface Props {
  ram: Ram;
}

export default function Computer({ ram }: Props) {
  const [wordDisplayBase, setWordDisplayBase] = useState(
    WordDisplayBase.Binary
  );

  return (
    <div className="panel-container">
      <TabControls
        items={wordDisplayBaseOptions}
        onChange={(idx) => setWordDisplayBase(idx as WordDisplayBase)}
        checkedIdx={wordDisplayBase as number}
        groupName="ram-display"
      />
      <div
        style={{ display: "flex", overflow: "auto", alignItems: "flex-start" }}
      >
        <code className="code-panel">
          {addresses.map((address) => (
            <span>{`${address}\n`}</span>
          ))}
        </code>
        <code className="code-panel">
          {addresses.map((address) => (
            <span>{`${get_ram_word(ram, address, wordDisplayBase)}\n`}</span>
          ))}
        </code>
      </div>
    </div>
  );
}
