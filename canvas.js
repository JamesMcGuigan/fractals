// DOCS: https://stackoverflow.com/a/51231100/748503

function resizeCanvas(canvas) {
    canvas.width  = window.innerWidth;
    canvas.height = window.innerHeight;
}
function drawCanvas(rgbaFn) {
    let timeStart = performance.now();

    let canvas = document.getElementById('rainbow');
    let ctx    = canvas.getContext('2d');
    resizeCanvas(canvas)

    let id     = ctx.getImageData(0, 0, canvas.width, canvas.height);
    let pixels = id.data;

    function setPx([x,y], [r,g,b,a=255]) {
        let offset = (y * id.width + x) * 4;
        pixels[offset    ] = r;
        pixels[offset + 1] = g;
        pixels[offset + 2] = b;
        pixels[offset + 3] = a;
        // ctx.putImageData(id, 0, 0);  // == 100x slowdown
    }

    // Loop over pixels
    for( let x=0; x<canvas.width; x++ ) {
        for( let y=0; y<canvas.height; y++ ) {
            let xy   = [x,y];
            let rgba = rgbaFn(xy, canvas);
            setPx( xy, rgba );
        }
    }
    ctx.putImageData(id, 0, 0);

    let timeTaken = performance.now() - timeStart;
    console.info(`drawCanvas(${rgbaFn.name}) - time: ${timeTaken.toFixed(0)}ms`);
}
