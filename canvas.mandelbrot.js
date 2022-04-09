// DOCS: https://www.codingame.com/playgrounds/2358/how-to-plot-the-mandelbrot-set/mandelbrot-set

let canvas = document.getElementById('mandelbrot');
const scale = 2.0
const depth = 25;

function mandelbrot(c) {
    let z = math.complex(0,0);
    for( var n = 0; n <= depth; n++ ) {
        z = math.add(math.multiply(z,z), c);
        let abs_z = math.abs(z)
        if( abs_z > 2*scale ) { break; }
        if( abs_z == 0      ) { return depth; }
    }
    return n;
}
function mandelbrotPercent([x,y]) {
    let xx = x/canvas.width  * 2*scale - scale - 1;
    let yy = y/canvas.height * 2*scale - scale;
    let c  = math.complex(xx,yy);
    let n  = mandelbrot(c);
    let pc = 1 - Math.min( n / depth, 1);

    // mandelbrotPercent.count += 1
    // mandelbrotPercent.sum   += n
    // mandelbrotPercent.mean   = mandelbrotPercent.sum / mandelbrotPercent.count;
    // mandelbrotPercent.max    = Math.max(n, mandelbrotPercent.max)
    return pc;
}
// mandelbrotPercent.count = 0;
// mandelbrotPercent.sum   = 0;
// mandelbrotPercent.mean  = 27;  // 27
// mandelbrotPercent.max   = 0;   // 256


function rgbaMandelbrotGray([x,y]) {
    let npc  = mandelbrotPercent([x,y]);
    let rbga = [ npc*255, npc*255, npc*255, 255 ];
    return rbga;
}
function rgbaMandelbrotColor([x,y]) {
    let npc  = mandelbrotPercent([x,y]);
    let rbg  = hsv_to_rgb(npc, 1, 255);
    let rbga = [ ...rbg, 255 ];
    if( npc == 1 ) { rbga = [255,255,255,255]; }
    if( npc == 0 ) { rbga = [0,0,0,255]; }

    return rbga;
}

(window.onresize = () => {
    // drawCanvas(canvas, rgbaMandelbrotGray);
    drawCanvas(canvas, rgbaMandelbrotColor);
    // console.log('rgbaMandelbrot.mean',rgbaMandelbrot.mean);
    // console.log('rgbaMandelbrot.max',rgbaMandelbrot.max);
})();
