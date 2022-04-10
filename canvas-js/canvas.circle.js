let canvas = document.getElementById('circle');
let ctx    = canvas.getContext('2d');

let px = {
    width:  canvas.width,
    height: canvas.height,
}
px = { ...px,
    radius: Math.min(px.width, px.height)/2 - 2,
    mid: { x: px.width/2, y: px.height/2 },
}
console.log(px);


// Background
ctx.beginPath();
ctx.strokeStyle = 'black';
ctx.fillRect(0, 0, px.width, px.height);
ctx.stroke();

// Border
ctx.beginPath();
ctx.strokeStyle = 'blue';
ctx.strokeRect(1, 1, px.width-2, px.height-2);
ctx.stroke();

// Circle
ctx.beginPath();
ctx.strokeStyle = 'red';
ctx.arc(px.mid.x, px.mid.y, px.radius, 0, 2 * Math.PI);
ctx.stroke();



