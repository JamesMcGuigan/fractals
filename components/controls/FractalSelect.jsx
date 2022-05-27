import React from 'react';

export default class FractalSelect extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }
    render() {
        return (
            <form className="toolbar top">
                <select id="fractal-name">
                    <option value="z = z^2 + c">Mandelbrot: z = z^2 + c</option>
                    <option value="x = y">x = y</option>
                </select>
            </form>
        )
    }
}