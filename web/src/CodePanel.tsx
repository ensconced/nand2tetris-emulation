import classnames from "classnames";
import React, { useEffect, useRef } from "react";

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
}

export default function CodePanel({
  filename,
  items,
  hoveredItemIdxs,
  selectedItemIdxs,
  onSpanMouseEnter,
  onSpanClick,
  onSpanMouseLeave,
  currentIdx,
}: Props) {
  const codeRef = useRef<HTMLElement>(null);

  useEffect(() => {
    if (selectedItemIdxs?.auto && selectedItemIdxs.filename === filename) {
      const firstHighlighedIdx = Math.min(...selectedItemIdxs.idxs);
      // scrollIntoView would make this slightly easier, but it doesn't work with `behavior: smooth`
      // for multiple elements simultaneously in chrome
      // https://bugs.chromium.org/p/chromium/issues/detail?id=833617
      if (codeRef.current) {
        const child = codeRef.current.children[firstHighlighedIdx];
        if (child instanceof HTMLElement) {
          codeRef.current.scrollTo({
            top: child.offsetTop,
            left: 0,
            behavior: "smooth",
          });
        }
      }
    }
  }, [selectedItemIdxs]);

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
