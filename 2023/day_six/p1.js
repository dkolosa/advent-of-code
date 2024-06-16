// Time:      7  15   30
// Distance:  9  40  200

// Time:        45     98     83     73
// Distance:   295   1734   1278   1210

var time = [45, 98, 83, 73];
var distance = [295, 1734, 1278, 1210];

var time2 = [45988373];
var distance2 = [295173412781210];
var beats = [0, 0, 0]
for (let index = 0; index < time2.length; index++) {
    const current_time = time2[index];
    const current_dist = distance2[index];
    let valid_distance = 0;
    for (let j = 1; j <= current_time; j++) {
        let cal_dist = (current_time - j) * j;
        if (distance2[index] < cal_dist) {
            valid_distance++;
         }
    }
    console.log(valid_distance)
    beats[index] = valid_distance;
}
console.log(beats.reduce((a,b) => a*b, 1))