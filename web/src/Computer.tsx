import React from "react";
import TabControls from "./TabControls";

const wordDisplayBaseOptions = ["binary", "decimal"];

interface Props {
  ram: string[];
  wordDisplayBaseIdx: number;
  onWordDisplayBaseIdxChange: (idx: number) => void;
}

export default function Computer({
  ram,
  wordDisplayBaseIdx,
  onWordDisplayBaseIdxChange: onChange,
}: Props) {
  return (
    <div className="panel-container">
      <TabControls
        items={wordDisplayBaseOptions}
        onChange={onChange}
        checkedIdx={wordDisplayBaseIdx}
        groupName="ram-display"
      />
      <code style={{ textAlign: "right" }} className="code-panel">
        {ram.map((line) => (
          <span>{line}</span>
        ))}
      </code>
    </div>
  );
}
