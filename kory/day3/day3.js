import { readFile } from "fs";

readFile('input.txt', "utf8", (_err, data) => {
    let validTrianglesCount = 0;
    let triangles = data.split(/\n/).map(row => row.matchAll(/(\d+)\s+(\d+)\s+(\d+)/g));
    triangles.forEach(triangle => {
        for (const side of triangle) {
            let a = parseInt(side[1]);
            let b = parseInt(side[2]);
            let c = parseInt(side[3]);

            let isValid = a + b > c && a + c > b && b + c > a;
            validTrianglesCount += isValid ? 1 : 0;
            console.log({ a, b, c, isValid });
        }
    });
    console.log(validTrianglesCount);
});