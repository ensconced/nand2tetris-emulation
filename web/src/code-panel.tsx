import React, { useEffect, useRef } from "react";

interface Props {
  items: Array<string>;
  mouseHoveredItemIdxs: Set<number>;
  autoHoveredItemIdxs: Set<number>;
  onSpanMouseOver(itemIdx: number): void;
  onSpanMouseLeave(): void;
}

export default function CodePanel({
  items,
  mouseHoveredItemIdxs,
  autoHoveredItemIdxs,
  onSpanMouseOver,
  onSpanMouseLeave,
}: Props) {
  const codeRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const firstHighlighedIdx = Math.min(...autoHoveredItemIdxs);
    codeRef.current?.children[firstHighlighedIdx]?.scrollIntoView({
      behavior: "smooth",
      block: "center",
    });
  }, [autoHoveredItemIdxs]);

  return (
    <code ref={codeRef} id="jack-code">
      {items.map((item, idx) => {
        return (
          <span
            key={idx}
            className={
              mouseHoveredItemIdxs.has(idx) || autoHoveredItemIdxs.has(idx)
                ? "highlighted"
                : ""
            }
            onMouseOver={() => onSpanMouseOver(idx)}
            onMouseLeave={() => onSpanMouseLeave()}
          >
            {item}
          </span>
        );
      })}
    </code>
  );
}
