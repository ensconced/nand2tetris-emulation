import classnames from "classnames";
import React, {
  forwardRef,
  useCallback,
  useEffect,
  useImperativeHandle,
  useRef,
} from "react";

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
  currentIdx?: number | undefined;
  onSpanMouseEnter(itemIdx: number): void;
  onSpanClick(itemIdx: number): void;
  onSpanMouseLeave(): void;
  breakpoints?: Record<number, boolean>;
  setBreakpoints?: (breakpoints: Record<number, boolean>) => void;
}

export interface CodePanelInstance {
  scrollTo: (itemIdx: number) => void;
}

export default forwardRef<CodePanelInstance, Props>(
  (
    {
      filename,
      items,
      hoveredItemIdxs,
      selectedItemIdxs,
      onSpanMouseEnter,
      onSpanClick,
      onSpanMouseLeave,
      currentIdx,
      breakpoints,
      setBreakpoints,
    },
    providedRef
  ) => {
    const codeRef = useRef<HTMLElement | null>(null);

    const scrollTo = useCallback((itemIdx: number) => {
      if (codeRef.current) {
        const child = codeRef.current.children[itemIdx];
        if (child instanceof HTMLElement) {
          // scrollIntoView would make this slightly easier, but it doesn't work with `behavior: smooth`
          // for multiple elements simultaneously in chrome
          // https://bugs.chromium.org/p/chromium/issues/detail?id=833617
          codeRef.current.scrollTo({
            top: child.offsetTop,
            left: 0,
            behavior: "smooth",
          });
        }
      }
    }, []);

    useImperativeHandle(providedRef, () => ({ scrollTo }), [scrollTo]);

    useEffect(() => {
      if (selectedItemIdxs?.auto && selectedItemIdxs.filename === filename) {
        const firstHighlighedIdx = Math.min(...selectedItemIdxs.idxs);
        scrollTo(firstHighlighedIdx);
      }
    }, [selectedItemIdxs, scrollTo]);

    return (
      <code className="code-panel" ref={codeRef} style={{ overflow: "auto" }}>
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
                breakpoint: breakpoints?.[idx],
              })}
              onMouseEnter={() => onSpanMouseEnter(idx)}
              onMouseLeave={onSpanMouseLeave}
              onContextMenu={(evt) => {
                if (setBreakpoints) {
                  evt.preventDefault();
                  setBreakpoints({
                    ...breakpoints,
                    [idx]: !breakpoints?.[idx],
                  });
                }
              }}
              onClick={() => onSpanClick(idx)}
            >
              {item}
            </span>
          );
        })}
      </code>
    );
  }
);
