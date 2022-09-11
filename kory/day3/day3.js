import { readFile } from "fs";

readFile('input.txt', "utf8", (_err, data) => {
    let TotalTriangles_Part1 = 0;
    let Triangles_Part1 = data.matchAll(/(\d+)\s+(\d+)\s+(\d+)/g);
    for (const side of Triangles_Part1) {
        let a = parseInt(side[1]);
        let b = parseInt(side[2]);
        let c = parseInt(side[3]);

        TotalTriangles_Part1 += isValidTriangle(a, b, c) ? 1 : 0;
    }

    let Triangles_Part2 = data.matchAll(/(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)/g);
    let TotalTriangles_Part2 = 0;

    for (const x of Triangles_Part2) {
        let a1 = parseInt(x[1]);
        let b1 = parseInt(x[4]);
        let c1 = parseInt(x[7]);
        TotalTriangles_Part2 += isValidTriangle(a1, b1, c1) ? 1 : 0;

        let a2 = parseInt(x[2]);
        let b2 = parseInt(x[5]);
        let c2 = parseInt(x[8]);
        TotalTriangles_Part2 += isValidTriangle(a2, b2, c2) ? 1 : 0;

        let a3 = parseInt(x[3]);
        let b3 = parseInt(x[6]);
        let c3 = parseInt(x[9]);
        TotalTriangles_Part2 += isValidTriangle(a3, b3, c3) ? 1 : 0;
    }

    console.table({ TotalTriangles_Part1, TotalTriangles_Part2 });
});

function isValidTriangle(a, b, c) {
    return a + b > c && a + c > b && b + c > a;
}