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
  selectedItemIdxs: (FileIdxs & { autoSelected: boolean }) | undefined;
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
}: Props) {
  const codeRef = useRef<HTMLElement>(null);

  useEffect(() => {
    if (
      selectedItemIdxs?.autoSelected &&
      selectedItemIdxs.filename === filename
    ) {
      const firstHighlighedIdx = Math.min(...selectedItemIdxs.idxs);
      codeRef.current?.children[firstHighlighedIdx]?.scrollIntoView({
        behavior: "smooth",
      });
    }
  }, [selectedItemIdxs]);

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
              selected:
                selectedItemIdxs?.filename === filename &&
                selectedItemIdxs.idxs.has(idx),
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
