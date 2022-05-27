import React from 'react';
import ZoomControls from "./controls/ZoomControls";
import FractalSelect from "./controls/FractalSelect";

export default class FractalExplorer extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }
    render() {
        return (
            <>
                {this.props.children}
                <FractalSelect/>
                <ZoomControls/>
            </>
        );
    }
}
