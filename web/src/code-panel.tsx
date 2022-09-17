import classnames from "classnames";
import React, { useEffect, useMemo, useRef } from "react";

export interface FileIdxs {
  filename: string;
  idxs: Set<number>;
}

interface Props {
  filename: string;
  items: Array<string>;
  hoveredItemIdxs: FileIdxs | undefined;
  mouseSelectedItemIdxs: FileIdxs | undefined;
  autoSelectedItemIdxs: FileIdxs | undefined;
  onSpanMouseEnter(itemIdx: number): void;
  onSpanClick(itemIdx: number): void;
  onSpanMouseLeave(): void;
}

export default function CodePanel({
  filename,
  items,
  hoveredItemIdxs,
  mouseSelectedItemIdxs,
  autoSelectedItemIdxs,
  onSpanMouseEnter,
  onSpanClick,
  onSpanMouseLeave,
}: Props) {
  const codeRef = useRef<HTMLElement>(null);

  useEffect(() => {
    if (autoSelectedItemIdxs?.filename === filename) {
      const firstHighlighedIdx = Math.min(...autoSelectedItemIdxs.idxs);
      codeRef.current?.children[firstHighlighedIdx]?.scrollIntoView({
        behavior: "smooth",
      });
    }
  }, [autoSelectedItemIdxs]);

  const selectedItemIdxs = useMemo(() => {
    const result = new Set<number>();
    [autoSelectedItemIdxs, mouseSelectedItemIdxs].forEach((selection) => {
      if (selection !== undefined && selection.filename === filename) {
        for (const val of selection.idxs.values()) {
          result.add(val);
        }
      }
    });
    return result;
  }, [autoSelectedItemIdxs, mouseSelectedItemIdxs]);

  return (
    <code className="code-panel" ref={codeRef}>
      {items.map((item, idx) => {
        return (
          <span
            key={idx}
            className={classnames({
              highlighted:
                hoveredItemIdxs?.filename === filename &&
                hoveredItemIdxs.idxs.has(idx),
              selected: selectedItemIdxs.has(idx),
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
