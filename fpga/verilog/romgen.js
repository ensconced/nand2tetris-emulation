const fs = require("fs");
const path = require("path");

const program = process.argv[2];
if (program === undefined) {
  throw new Error("no program");
}

const sourcePath = path.resolve(__dirname, "./programs", program);

const lines = fs
  .readFileSync(sourcePath, "utf-8")
  .split("\n")
  .map((line) => {
    return line.match(/[01\s]*/)[0];
  })
  .filter((line) => line.length > 0)
  .map((line) => line.replace(/\s/g, ""));

function nextWord() {
  const instruction = lines.shift();
  return instruction ? instruction.replace(/\s/g, "") : "0000000000000000";
}

function romLiteral() {
  let result = '';
  for (let i = 0; i < wordCount; i++) {
    result += `${nextWord()}\n`;
  }
  return result;
}

const power = 15;
const wordCount = 2 ** power;
console.log(romLiteral());
