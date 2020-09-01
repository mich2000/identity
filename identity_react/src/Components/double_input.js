import React from 'react';

export default class DoubleInput extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            input1 : "",
            input2 : "",
            name : this.props.name
        };
        this.click_callback = this.click_callback.bind(this);
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    click_callback(event){
        this.props.input_callback(this.state.input1, this.state.input2);
        this.setState({
            input1 : "",
            input2 : ""
        });
        event.preventDefault();
        event.stopPropagation();
    }

    render() {
        if(this.props.hideInput !== "") {
            return (
                <form onSubmit={(e) => this.click_callback(e)}>
                    <div>
                        <input type="text" className="form-control" placeholder={this.props.firstPlaceholder || ""} value={this.state.input1} name="input1" onChange={(e) => this.change_handler(e)} required/>
                        <input type="text" className="form-control" placeholder={this.props.secondPlaceholder || ""} value={this.state.input2} name="input2" onChange={(e) => this.change_handler(e)} required/>
                        <div className="input-group-btn">
                            <button className="btn btn-default" type="submit">
                                {this.state.name}
                            </button>
                        </div>
                    </div>
                </form>
            );
        }
        return (
            <form onSubmit={(e) => this.click_callback(e)}>
                <div>
                    <input type="text" className="form-control" placeholder={this.props.firstPlaceholder || ""} value={this.state.input1} name="input1" onChange={(e) => this.change_handler(e)} type="password" autoComplete="new-password" required/>
                    <input type="text" className="form-control" placeholder={this.props.secondPlaceholder || ""} value={this.state.input2} name="input2" onChange={(e) => this.change_handler(e)} type="password" autoComplete="new-password" required/>
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