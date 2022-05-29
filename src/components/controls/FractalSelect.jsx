import React from 'react';

export default class FractalSelect extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            value: props.fractal || 'colormap'
        };
        this.props.onChange(this.state.value)
    }
    render() {
        return (
            <form className="toolbar top">
                <select
                    id="fractal-name"
                    onChange={event => this.props.onChange(event.target.value)}
                    value={this.state.value}
                >
                    <option value="colormap">colormap</option>
                    <option value="z = z^2 + c">Mandelbrot: z = z^2 + c</option>
                </select>
            </form>
        )
    }
}