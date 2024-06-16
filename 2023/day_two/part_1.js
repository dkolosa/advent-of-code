const fs = require('node:fs');

var data = fs.readFileSync('min_input.txt').toString().split("\n");

const max_red = 12;
const max_green = 13;
const max_blue = 14;

var game_id_cnt = 0

for (let index = 0; index < data.length; index++) {
    
    var game_id = parseInt(data[index].match(/(\d+):/g)[0].slice(0,-1));
    var draws = data[index].substring(data[index].indexOf(':')+1)
    var rows = draws.split(";")
    
    var min_red= 0
    var min_blue = 0
    var min_green = 0

    var valid_games = rows.map(isvalid)
    if (!valid_games.includes(false)){
        game_id_cnt += game_id;
    }
}
console.log(game_id_cnt)

function isvalid(element){
    var colors = element.split(",");
    var green = 0;
    var blue = 0; 
    var red = 0;



    for (let index = 0; index < colors.length; index++) {
        if (colors[index].match(/ \w/g)[1] == " g"){
            green = parseInt(colors[index].match(/\d+/g)[0]);
            if (green > min_green) {
                min_green = green;
            }
        } 
        if (colors[index].match(/ \w/g)[1] == " r"){
           red = parseInt(colors[index].match(/\d+/g)[0])
           if (red > min_red){
                min_red = red
           }
        }
         if (colors[index].match(/ \w/g)[1] == " b"){
            blue = parseInt(colors[index].match(/\d+/g)[0])
            if (blue > min_blue) {
                min_blue = blue
            }
        } 
    }

    console.log(min_red, min_green, min_blue)
    pow = power(min_red, min_green, min_blue);
    if (red <= max_red && blue <= max_blue && green <= max_green)
    {   
        return true;
    }
    return false;

    
}

function power(red, green, blue){
    return red * green * blue;
}