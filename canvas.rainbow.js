// DOCS: https://stackoverflow.com/a/51231100/748503

function resizeCanvas(canvas) {
    canvas.width  = window.innerWidth;
    canvas.height = window.innerHeight;
}
function drawCanvas(callback) {
    let timeStart = performance.now();

    let canvas = document.getElementById('rainbow');
    let ctx    = canvas.getContext('2d');
    resizeCanvas(canvas)

    let id     = ctx.getImageData(0, 0, canvas.width, canvas.height);
    let pixels = id.data;

    function setPx([x,y], [r,g,b,a=255]) {
        let off = (y * id.width + x) * 4;
        pixels[off    ] = r;
        pixels[off + 1] = g;
        pixels[off + 2] = b;
        pixels[off + 3] = a;
        // ctx.putImageData(id, 0, 0);  // == 100x slowdown
    }
    function getRgba([x,y]) {
        let r = x * 255/canvas.width;
        let g = y * 255/canvas.height;
        let b = (r + g) / 2;
        let a = 255;
        return [r,g,b,a];
    }

    callback(canvas, id, pixels, { setPx, getRgba })

    ctx.putImageData(id, 0, 0);

    let timeTaken = performance.now() - timeStart;
    console.info('draw(): timeTaken', timeTaken);
}

function drawRainbow(canvas, id, pixels, fn) {
    for( let x=0; x<canvas.width; x++ ) {
        for( let y=0; y<canvas.height; y++ ) {
            let xy   = [x,y];
            let rgba = fn.getRgba(xy);
            fn.setPx( xy, rgba );
        }
    }
}

(window.onresize = () => {
    drawCanvas(drawRainbow);
})();
