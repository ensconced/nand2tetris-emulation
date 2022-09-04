import classnames from "classnames";
import React, { useEffect, useMemo, useRef } from "react";

interface Props {
  items: Array<string>;
  hoveredItemIdxs: Set<number>;
  mouseSelectedItemIdxs: Set<number>;
  autoSelectedItemIdxs: Set<number>;
  onSpanMouseEnter(itemIdx: number): void;
  onSpanClick(itemIdx: number): void;
  onSpanMouseLeave(): void;
}

export default function CodePanel({
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
    const firstHighlighedIdx = Math.min(...autoSelectedItemIdxs);
    codeRef.current?.children[firstHighlighedIdx]?.scrollIntoView({
      behavior: "smooth",
    });
  }, [autoSelectedItemIdxs]);

  const selectedItemIdxs = useMemo(
    () => new Set([...autoSelectedItemIdxs, ...mouseSelectedItemIdxs]),
    [autoSelectedItemIdxs, mouseSelectedItemIdxs]
  );

  return (
    <div className="code-wrapper">
      <code className="code-panel" ref={codeRef}>
        {items.map((item, idx) => {
          return (
            <span
              key={idx}
              className={classnames({
                highlighted: hoveredItemIdxs.has(idx),
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
      <code className="footer">
        <span style={{ color: "#ff79c6" }} className="footer-item">
          hovered: {hoveredItemIdxs.size}
        </span>
        <span style={{ color: "#f8f8f2" }} className="footer-item">
          selected: {selectedItemIdxs.size}
        </span>
      </code>
    </div>
  );
}
