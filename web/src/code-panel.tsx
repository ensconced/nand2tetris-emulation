import classnames from "classnames";
import React, { useEffect, useRef } from "react";

interface Props {
  items: Array<string>;
  hoveredItemIdxs: Set<number>;
  mouseSelectedItemIdxs: Set<number>;
  autoSelectedItemIdxs: Set<number>;
  onSpanMouseEnter(itemIdx: number): void;
  onSpanClick(itemIdx: number): void;
  onSpanMouseLeave(): void;
  footerItems: Array<string | undefined>;
}

export default function CodePanel({
  items,
  hoveredItemIdxs,
  mouseSelectedItemIdxs,
  autoSelectedItemIdxs,
  onSpanMouseEnter,
  onSpanClick,
  onSpanMouseLeave,
  footerItems,
}: Props) {
  const codeRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const firstHighlighedIdx = Math.min(...autoSelectedItemIdxs);
    codeRef.current?.children[firstHighlighedIdx]?.scrollIntoView({
      behavior: "smooth",
    });
  }, [autoSelectedItemIdxs]);

  return (
    <div className="code-wrapper">
      <code className="code-panel" ref={codeRef}>
        {items.map((item, idx) => {
          return (
            <span
              key={idx}
              className={classnames({
                highlighted: hoveredItemIdxs.has(idx),
                selected:
                  autoSelectedItemIdxs.has(idx) ||
                  mouseSelectedItemIdxs.has(idx),
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
        {footerItems.map((footerItem) => (
          <span className="footer-item">{footerItem ?? ""}</span>
        ))}
      </code>
    </div>
  );
}
