function rgbaRainbow([x,y], canvas) {
    let r = x * 255/canvas.width;
    let g = y * 255/canvas.height;
    let b = (r + g) / 2;
    let a = 255;
    return [r,g,b,a];
}

(window.onresize = () => {
    drawCanvas(rgbaRainbow);
})();
