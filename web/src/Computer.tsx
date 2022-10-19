import React from "react";
import TabControls from "./TabControls";
import { FixedSizeList, ListChildComponentProps } from "react-window";

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
  const Row = ({ index, style }: ListChildComponentProps) => {
    const word = ram[index]!;
    return <span style={style}>{`${word}\n`}</span>;
  };

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
