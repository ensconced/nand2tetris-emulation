import React from "react";
import TabControls from "./TabControls";
import { FixedSizeList, ListChildComponentProps } from "react-window";
import { WordDisplayBase } from "../../web-emulator/pkg/web_emulator";

const wordDisplayBaseOptions = ["binary", "decimal"];

interface Props {
  wordDisplayBase: WordDisplayBase;
  onWordDisplayBaseIdxChange: (idx: number) => void;
  getRamWord: (addr: number) => string;
}

export default function Computer({
  wordDisplayBase,
  onWordDisplayBaseIdxChange: onChange,
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
        checkedIdx={wordDisplayBaseIdx}
        groupName="ram-display"
      />
      <code style={{ textAlign: "right" }} className="code-panel">
        <FixedSizeList
          height={1000}
          width={200}
          // This is font-size * line-height (values copied from reset.css), to match the height of all the other spans on the page.
          itemSize={13 * 1.2}
          itemCount={32768}
        >
          {Row}
        </FixedSizeList>
      </code>
    </div>
  );
}
