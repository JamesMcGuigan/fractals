import React from 'react';
import styles from "./ZoomControls.module.less";

export default class ZoomControls extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }
    render() {
        return (
            <div className={styles.ZoomControls}>
                <form className={styles.bottom}>
                    <span>
                        <label htmlFor='x-coord'>X:</label>
                        <input type="number" id="x-coord" placeholder="x" value="0.25" step="0.25"/>
                    </span>
                    <span>
                        <label htmlFor='y-coord'>Y:</label>
                        <input type="number" id="y-coord" placeholder="y" value="0.0" step="0.25"/>
                    </span>
                    <span>
                        <label htmlFor='zoom'>Zoom:</label>
                        <input type="number" id="zoom" placeholder="zoom" value="2.0" step="0.1"/>
                    </span>
                    <span>
                        <label htmlFor='speed'>Speed:</label>
                        <input type="number" id="speed" placeholder="speed" value="0.0" step="0.1"/>
                    </span>
                </form>
            </div>
        )
    }
}