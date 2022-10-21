import React from "react";
import TabControls from "./TabControls";
import { FixedSizeList, ListChildComponentProps } from "react-window";
import { WordDisplayBase } from "../../web-emulator/pkg/web_emulator";
import { lineHeight } from "./CodePanel";

const wordDisplayBaseOptions = ["binary", "decimal"];

interface Props {
  wordDisplayBase: WordDisplayBase;
  onWordDisplayBaseChange: (idx: number) => void;
  getRamWord: (addr: number) => string;
}

export default function Computer({
  wordDisplayBase,
  onWordDisplayBaseChange: onChange,
  getRamWord,
}: Props) {
  function Row({ index, style }: ListChildComponentProps) {
    return <span style={style}>{`${getRamWord(index)}\n`}</span>;
  }

  return (
    <div className="panel-container">
      <TabControls
        items={wordDisplayBaseOptions}
        onChange={onChange}
        checkedIdx={wordDisplayBase as number}
        groupName="ram-display"
      />
      <code style={{ textAlign: "right" }} className="code-panel">
        <FixedSizeList
          height={1000}
          width={150}
          itemSize={lineHeight}
          itemCount={32768}
        >
          {Row}
        </FixedSizeList>
      </code>
    </div>
  );
}
