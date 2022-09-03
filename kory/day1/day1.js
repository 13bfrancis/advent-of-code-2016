const fs = require("fs");

fs.readFile("input.txt", "utf8", (err, data) => {
    data = data.split(', ');

    let x = 0;
    let y = 0;
    let directions = [{ x: 0, y: 1 }, { x: 1, y: 0 }, { x: 0, y: -1 }, { x: -1, y: 0 }];
    let currentDirection = 0;
    let direction = null;

    for (const dir of data) {
        const re = /(\w{1})(\d+)/;
        let turn = dir.match(re)[1];
        let move = dir.match(re)[2];

        if (turn == 'L') {
            currentDirection = mod(currentDirection - 1, 4);
        } else {
            currentDirection = mod(currentDirection + 1, 4);
        }

        direction = directions[currentDirection];
        x += direction.x * move;
        y += direction.y * move;
    }

    console.log(`Day 1 Part 1: ${x+y}`);
});

function mod(n, m) {
    return ((n % m) + m) % m;
}
