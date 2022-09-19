async function parseFile(): Promise<{ part1: number[][]; part2: number[][] }> {
  const fileContent = await Deno.readTextFile("inputs.txt");
  const part1 = fileContent
    .split("\n")
    .map(
      (line) => line.match(/(\d{1,})/g)?.map((number) => parseInt(number)) ?? []
    );

  const col1: number[] = [];
  const col2: number[] = [];
  const col3: number[] = [];
  let counter = 0;

  for (const row of part1) {
    for (const col of row) {
      if (counter % 3 === 0) col1.push(col);
      if (counter % 3 === 1) col2.push(col);
      if (counter % 3 === 2) col3.push(col);

      counter++;
    }
  }

  const part2temp = [...col1, ...col2, ...col3];
  const part2: number[][] = [];

  const chunkSize = 3;
  for (let i = 0; i < part2temp.length; i += chunkSize) {
    part2.push(part2temp.slice(i, i + chunkSize));
  }

  return { part1, part2 };
}

function isTriangle(triangle: number[]) {
  const sum1 = triangle[0] + triangle[1] > triangle[2];
  const sum2 = triangle[1] + triangle[2] > triangle[0];
  const sum3 = triangle[0] + triangle[2] > triangle[1];

  return sum1 && sum2 && sum3;
}

const { part1, part2 } = await parseFile();

console.log(part1.filter((triangle) => isTriangle(triangle)).length);
console.log(part2.filter((triangle) => isTriangle(triangle)).length);
