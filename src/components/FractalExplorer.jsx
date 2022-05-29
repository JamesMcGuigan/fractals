import React from 'react';
import ZoomControls from "./controls/ZoomControls";
import FractalSelect from "./controls/FractalSelect";

export default class FractalExplorer extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            fractal: ""
        };
    }
    setState(state, callback) {
        super.setState(state, callback);
    }
    render() {
        return (
            <>
                {React.cloneElement(this.props.children, this.state)}
                <FractalSelect onChange={value => this.setState({ fractal: value }) }/>
                <ZoomControls/>
            </>
        );
    }
}
