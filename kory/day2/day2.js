import { readFile } from "fs";

const keypad = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

readFile('input.txt', "utf8", (_err, data) => {
    let instructions = data.split(/\n/).map(row => row.split(''));

    let xStart = 1;
    let yStart = 1;
    let bathroomCode = '';
    
    instructions.forEach(row => {
        row.forEach(op => {
            switch (op.toUpperCase()) {
                case 'U':
                    yStart = Math.max(0, yStart - 1);
                    break;
                case 'D':
                    yStart = Math.min(yStart + 1, 2);
                    break;
                case 'L':
                    xStart = Math.max(0, xStart -1);
                    break;
                case 'R':
                    xStart = Math.min(xStart + 1, 2);
                    break;
            }
        });
        
        bathroomCode += keypad[yStart][xStart];
    });

    console.table({bathroomCode});
});
