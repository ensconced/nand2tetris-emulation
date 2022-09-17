import React from "react";
import { NodeInfoId } from "./index";

interface Props {
  hoveredJackNode: NodeInfoId | undefined;
  selectedJackNode: NodeInfoId | undefined;
}

export default function Footer({ hoveredJackNode, selectedJackNode }: Props) {
  return (
    <code className="footer">
      <span style={{ color: "#f8f8f2" }} className="footer-item">
        hovered node tokens: {hoveredJackNode?.filename ?? ""}{" "}
        {hoveredJackNode?.node.token_range.start} -{" "}
        {hoveredJackNode?.node.token_range.end}
      </span>
      <span style={{ color: "#ff79c6" }} className="footer-item">
        selected node tokens: {selectedJackNode?.filename ?? ""}{" "}
        {selectedJackNode?.node.token_range.start} -{" "}
        {selectedJackNode?.node.token_range.end}
      </span>
    </code>
  );
}
