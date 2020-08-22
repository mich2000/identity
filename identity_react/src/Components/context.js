import React from 'react'
import User from './user'

class Context extends React.Component {
    render() {
        return (
            <div>
                <h1>Rust React identity application</h1>
                <User/>
            </div>
        );
    }
}

export default Context;