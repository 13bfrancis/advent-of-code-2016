const fs = require("fs");

fs.readFile("input.txt", "utf8", (_err, data) => {
    let x = 0;
    let y = 0;
    let directions = [{ x: 0, y: 1 }, { x: 1, y: 0 }, { x: 0, y: -1 }, { x: -1, y: 0 }];
    let currentDirection = 0;
    let locations = [];
    let firstCrossedPath = null;

    for (const dir of data.split(', ')) {
        const re = /(\w{1})(\d+)/;
        let turn = dir.match(re)[1];
        let move = dir.match(re)[2];
        let arrShift = (turn == 'L' ? -1 : 1);

        currentDirection = mod(currentDirection + arrShift, 4);

        let direction = directions[currentDirection];

        let xMove = direction.x * move;
        let yMove = direction.y * move;

        for (let i = 1; i <= Math.abs(xMove); i++) {
            x += Math.sign(xMove);

            firstCrossedPath = firstCrossedPath ?? checkLocations(locations, x, y);
        }

        for (let i = 1; i <= Math.abs(yMove); i++) {
            y += Math.sign(yMove);

            firstCrossedPath = firstCrossedPath ?? checkLocations(locations, x, y);
        }
    }

    console.table({ 'Part 1': x + y, 'Part 2': firstCrossedPath });
});

function mod(n, m) {
    return ((n % m) + m) % m;
}

function checkLocations(locations, x, y) {
    if (locations.filter((l) => l.x == x && l.y == y).length > 0) {
        return x + y;
    } else {
        locations.push({ x, y });
    }
}