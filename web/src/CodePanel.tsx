import classnames from "classnames";
import React, { useEffect, useRef } from "react";
import { FixedSizeList, ListChildComponentProps } from "react-window";
import AutoSizer from "react-virtualized-auto-sizer";

export interface FileIdxs {
  filename: string;
  idxs: Set<number>;
}

export type InteractedItemIdxs = (FileIdxs & { auto: boolean }) | undefined;
export type InteractedInstructionIdxs =
  | Omit<NonNullable<InteractedItemIdxs>, "filename">
  | undefined;

interface Props {
  id: string;
  filename: string;
  items: Array<string>;
  hoveredItemIdxs: FileIdxs | undefined;
  selectedItemIdxs: InteractedItemIdxs;
  currentIdx: number | undefined;
  onSpanMouseEnter(itemIdx: number): void;
  onSpanClick(itemIdx: number): void;
  onSpanMouseLeave(): void;
  windowed: boolean;
}

// This is font-size * line-height (values copied from reset.css), to match the height of all the other spans on the page.
export const lineHeight = 13 * 1.2;

export default function CodePanel({
  filename,
  items,
  hoveredItemIdxs,
  selectedItemIdxs,
  onSpanMouseEnter,
  onSpanClick,
  onSpanMouseLeave,
  currentIdx,
  windowed,
}: Props) {
  const codeRef = useRef<HTMLElement>(null);

  useEffect(() => {
    if (selectedItemIdxs?.auto && selectedItemIdxs.filename === filename) {
      const firstHighlighedIdx = Math.min(...selectedItemIdxs.idxs);
      // scrollIntoView would make this slightly easier, but it doesn't work with `behavior: smooth`
      // for multiple elements simultaneously in chrome
      // https://bugs.chromium.org/p/chromium/issues/detail?id=833617
      if (codeRef.current) {
        let offsetTop: number | undefined;
        if (windowed) {
          offsetTop = lineHeight * firstHighlighedIdx;
        } else {
          const child = codeRef.current.children[firstHighlighedIdx];
          if (child instanceof HTMLElement) {
            offsetTop = child.offsetTop;
          }
        }
        if (offsetTop !== undefined) {
          codeRef.current.scrollTo({
            top: offsetTop,
            left: 0,
            behavior: "smooth",
          });
        }
      }
    }
  }, [selectedItemIdxs]);

  if (windowed) {
    const Row = ({ index, style }: ListChildComponentProps) => {
      const item = items[index]!;
      return (
        <span
          style={style}
          className={classnames({
            hovered:
              hoveredItemIdxs?.filename === filename &&
              hoveredItemIdxs.idxs.has(index),
            selected:
              selectedItemIdxs?.filename === filename &&
              selectedItemIdxs.idxs.has(index),
            current: currentIdx === index,
          })}
          onMouseEnter={() => onSpanMouseEnter(index)}
          onMouseLeave={onSpanMouseLeave}
          onClick={() => onSpanClick(index)}
        >
          {item}
        </span>
      );
    };
    return (
      <code className="code-panel">
        <AutoSizer>
          {({ height, width }) => (
            <FixedSizeList
              outerRef={codeRef}
              height={height}
              width={width}
              itemCount={items.length}
              itemSize={lineHeight}
              overscanCount={20}
            >
              {Row}
            </FixedSizeList>
          )}
        </AutoSizer>
      </code>
    );
  }

  return (
    <code className="code-panel" ref={codeRef}>
      {items.map((item, idx) => {
        return (
          <span
            key={idx}
            className={classnames({
              hovered:
                hoveredItemIdxs?.filename === filename &&
                hoveredItemIdxs.idxs.has(idx),
              selected:
                selectedItemIdxs?.filename === filename &&
                selectedItemIdxs.idxs.has(idx),
              current: currentIdx === idx,
            })}
            onMouseEnter={() => onSpanMouseEnter(idx)}
            onMouseLeave={onSpanMouseLeave}
            onClick={() => onSpanClick(idx)}
          >
            {item}
          </span>
        );
      })}
    </code>
  );
}
