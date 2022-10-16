import { NodeInfo } from "../bindings/NodeInfo";

export interface NodeInfoId {
  filename: string;
  node: NodeInfo;
}

export interface FileIdx {
  filename: string;
  idx: number;
}
