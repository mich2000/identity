import React from 'react';

export default class Input extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            input : "",
            name : this.props.name
        };
        this.click_callback = this.click_callback.bind(this);
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    click_callback(event){
        this.props.input_callback(this.state.input);
        this.setState({input : ""});
        event.preventDefault();
        event.stopPropagation();
    }

    render() {
        return (
            <form onSubmit={(e) => this.click_callback(e)}>
                <div className="input-group">
                    <input type="text" className="form-control" value={this.state.input} name="input" onChange={(e) => this.change_handler(e)} required/>
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