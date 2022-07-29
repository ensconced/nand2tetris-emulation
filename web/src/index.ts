import "./styles/reset.css";
import { ParserVizData } from "../../bindings/ParserVizData";
import rawData from "../../parser_viz_data.json";

const data = rawData as ParserVizData;

const main = document.getElementById("main");
if (main === null) {
  throw new Error("main element is missing");
}

console.log(data.source);
