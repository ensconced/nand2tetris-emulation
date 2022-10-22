/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { CSSProperties, ReactElement, useState } from "react";
import TabControls from "./TabControls";
import { FixedSizeList, ListChildComponentProps } from "react-window";
import {
  get_ram_word,
  Ram,
  WordDisplayBase,
} from "../../web-emulator/pkg/web_emulator";
import { lineHeight } from "./CodePanel";
import AutoSizer from "react-virtualized-auto-sizer";

const wordDisplayBaseOptions = ["binary", "decimal", "binary blocks"];

const ramDepth = 32768;

interface Data {
  ram: Ram;
  wordDisplayBase: WordDisplayBase;
}

// This InnerElement/row setup is pretty hacky - I'm abusing react-window in order to be able to
// have two separate code panels (one for the addresses, and one the corresponding words in RAM),
// which scroll together as a unit. react-window expects `row` to be a react component (i.e. to
// return a react element), but instead I return an array containing two react elements - one for
// each of the code panels.
function InnerElement({
  style,
  children,
}: {
  style: CSSProperties;
  children: Array<ReactElement<ListChildComponentProps<Data>>>;
}) {
  const properChildren = children.map((child) => row(child.props));
  return (
    <div style={{ display: "flex" }}>
      <code className="code-panel" style={style}>
        <div style={{ height: "100%", width: "100%", position: "relative" }}>
          {properChildren.map(([addr]) => addr)}
        </div>
      </code>
      <code className="code-panel" style={style}>
        <div style={{ height: "100%", width: "100%", position: "relative" }}>
          {properChildren.map(([, word]) => word)}
        </div>
      </code>
    </div>
  );
}

function row({
  index,
  style,
  data: { ram, wordDisplayBase },
}: ListChildComponentProps<Data>) {
  return [
    <span style={style}>{`${index}\n`}</span>,
    <span style={style}>{`${get_ram_word(
      ram,
      index,
      wordDisplayBase
    )}\n`}</span>,
  ];
}

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
      <div style={{ flex: 1 }}>
        <AutoSizer>
          {({ height, width }) => (
            <FixedSizeList
              height={height}
              width={width}
              itemSize={lineHeight}
              itemCount={ramDepth}
              innerElementType={InnerElement}
              itemData={{ ram, wordDisplayBase }}
              overscanCount={20}
            >
              {row as any}
            </FixedSizeList>
          )}
        </AutoSizer>
      </div>
    </div>
  );
}
