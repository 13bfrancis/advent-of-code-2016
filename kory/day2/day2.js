import { readFile } from "fs";

const keypad = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

const newKeyPad = [
    [null, null, '1', null, null],
    [null, '2', '3', '4', null],
    ['5', '6', '7', '8', '9'],
    [null, 'A', 'B', 'C', null],
    [null, null, 'D', null, null]
]
readFile('input.txt', "utf8", (_err, data) => {
    let instructions = data.split(/\n/).map(row => row.split(''));

    let pos1 = {x: 1, y: 1};
    let pos2 = {x: 0, y: 2};
    
    let bathroomCode = '';
    let newBathroomCode = '';

    instructions.forEach(row => {
        row.forEach(op => {
            switch (op.toUpperCase()) {
                case 'U':
                    pos1.y = Math.max(0, --pos1.y);
                    pos2.y = Math.max(0, --pos2.y);
                    pos2.y = newKeyPad[pos2.y][pos2.x] ? pos2.y : ++pos2.y;
                    break;
                case 'D':
                    pos1.y = Math.min(++pos1.y, 2);
                    pos2.y = Math.min(++pos2.y, 4);
                    pos2.y = newKeyPad[pos2.y][pos2.x] ? pos2.y : --pos2.y;
                    break;
                case 'L':
                    pos1.x = Math.max(0, --pos1.x);
                    pos2.x = Math.max(0, --pos2.x);
                    pos2.x = newKeyPad[pos2.y][pos2.x] ? pos2.x : ++pos2.x;
                    break;
                case 'R':
                    pos1.x = Math.min(++pos1.x, 2);
                    pos2.x = Math.min(++pos2.x, 4);
                    pos2.x = newKeyPad[pos2.y][pos2.x] ? pos2.x : --pos2.x;
                    break;
            }
        });

        bathroomCode += keypad[pos1.y][pos1.x];
        newBathroomCode += newKeyPad[pos2.y][pos2.x];
    });

    console.table({ bathroomCode, newBathroomCode });
});
