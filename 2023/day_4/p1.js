const fs = require('node:fs');

var data = fs.readFileSync('input.txt').toString().split("\n");
var count_array = [];
for (let index = 0; index < data.length; index++) {
    let dataline = data[index];
    var line = dataline.split(':')[1];
    line = line.split('|');

    let win_set = new Set(line[0].split(' ').filter(Boolean));
    card_list = line[1].split(' ').filter(Boolean);

    var count = 0;

    card_list.forEach(element => {
        if (win_set.has(element)){
            if (count == 0)
                count++;
            else
                count = count * 2
        }

    });
    // console.log(count)
    count_array.push(count)
}

let sum = count_array.reduce(add, 0)

function add(acc, a) {
    return acc + a;
}
console.log(sum)