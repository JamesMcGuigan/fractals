let canvas = document.getElementById('mandelbrot');
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
