import React from "react";

interface Props {
  checkedIdx: number;
  onChange: (idx: number) => void;
  items: string[];
  groupName: string;
}

export default function TabControls({
  checkedIdx,
  onChange,
  items,
  groupName,
}: Props) {
  return (
    <fieldset style={{ flex: "0 0 auto" }}>
      {items.map((title, idx) => {
        return (
          <React.Fragment key={title}>
            <input
              id={`${groupName}-${idx}`}
              type="radio"
              name={groupName}
              checked={checkedIdx === idx}
              onChange={() => onChange(idx)}
            />
            <label htmlFor={`${groupName}-${idx}`}>{title}</label>
          </React.Fragment>
        );
      })}
    </fieldset>
  );
}
