import React from 'react';

class Input extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            input : "",
            name : this.props.name
        };
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    render() {
        return (
            <form onSubmit={(e) => this.props.input_callback(e,this.state.input)}>
                <div className="input-group">
                    <input type="text" className="form-control" name="input" onChange={(e) => this.change_handler(e)}/>
                    <div className="input-group-btn">
                        <button className="btn btn-default" type="submit">
                            {this.state.name}
                        </button>
                    </div>
                </div>
            </form>
        );
    }
}

export default Input;