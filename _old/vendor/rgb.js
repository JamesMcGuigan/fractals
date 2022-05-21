function rgb_to_hsv(r, g, b) {
    let maxc = Math.max(r, g, b);
    let minc = Math.min(r, g, b);
    let v   = maxc
    if (minc == maxc) { return (0, 0, v); }
    let diff = maxc - minc;
    let s    = diff / maxc;
    let rc = (maxc - r) / diff;
    let gc = (maxc - g) / diff;
    let bc = (maxc - b) / diff;

    let h
    if      ( r == maxc ) { h = bc - gc; }
    else if ( g == maxc ) { h = 2.0 + rc - bc; }
    else {
        h = 4.0 + gc - rc
        h = (h / 6.0) % 1.0 // this calculates only the fractional part of h/6
    }
    return [h, s, v];
}

function hsv_to_rgb(h, s, v) {
    if( s == 0.0 ) { return [v, v, v]; }
    let i = Math.floor(h * 6.0);  // int
    let f = (h*6.0) - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s*f);
    let t = v * (1.0 - s*(1.0 - f));
    if( (i % 6) === 0 ) { return [v, t, p]; }
    if( (i % 6) === 1 ) { return [q, v, p]; }
    if( (i % 6) === 2 ) { return [p, v, t]; }
    if( (i % 6) === 3 ) { return [p, q, v]; }
    if( (i % 6) === 4 ) { return [t, p, v]; }
    if( (i % 6) === 5 ) { return [v, p, q]; }
    return [0, 0, 0];
}
