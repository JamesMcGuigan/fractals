let canvas = document.getElementById('rainbow');

function rgbaRainbow([x,y]) {
    let r = 255 * x/canvas.width;
    let g = 255 * y/canvas.height;
    let b = (r + g) / 2;
    let a = 255;
    return [r,g,b,a];
}

(window.onresize = () => {
    drawCanvas(canvas, rgbaRainbow);
})();
