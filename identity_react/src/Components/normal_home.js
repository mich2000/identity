import React from 'react';
import Registration from './registration';
import Login from './login';

export function About() {
    return (
        <div>
            <h1>About page of the website</h1>
            <p>
                This react application has been made to demonstrate the rust web api application this will also give the possibility to register and login users. You can add and delete unique flags.
            </p>
        </div>
    );
}

export default class UnauthenticatedHome extends React.Component {
    constructor(props) {
        super(props);
        this.state ={
            error : ""
        }
        this.log_error = this.log_error.bind(this);
    }

    log_error(err_msg) {
        console.error(err_msg);
        this.setState({ error : err_msg });
    }

    render() {
        return (
            <div>
                <h1>Rust identity application</h1>
                <span className="font-weight-bold text-danger">{this.state.error}</span>
                <Login login_callback = {this.props.login_callback} log_error={this.log_error}/>
                <Registration log_error={this.log_error}/>
            </div>
        );
    }
}