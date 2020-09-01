import React from 'react';
import api_functions from '../api';
import email from "../email";

export default class Registration extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            email : "",
            password : "",
            confirm_password : ""
        };
        this.change_handler = this.change_handler.bind(this);
        this.registration = this.registration.bind(this);
        this.registration_okay = this.registration_okay.bind(this);
     }

    registration_okay() {
        if(!email.control_email(this.state.email)) {
            this.props.log_error("The email is not right.");
            return false;
        }
        if(this.state.confirm_password !== this.state.password) {
            this.props.log_error("Password and confirm password aren't the same.");
            return false;
        }
        return true;
    }
    
     registration(e) {
        if(this.registration_okay()) {
            let opties = api_functions.method_post();
            opties.body = JSON.stringify({
                email : this.state.email,
                password : this.state.password,
                confirm_password : this.state.password
            });
            fetch(api_functions.get_api() + "/user/registration", opties)
            .then((api_call) => api_call.json())
            .then((api_call) => {
                if(api_call.ok) {
                    alert("Registration has succeeded😀.");
                } else {
                    this.props.log_error(api_call.error);
                }
            }).catch(() => {
                this.props.log_error("Could not register the account");
            });
        }
        e.preventDefault();
        e.stopPropagation();
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    render() {
        return (
            <form className="col-md-6" onSubmit={(e) => this.registration(e)}>
                <h2>Registration</h2>
                <div className="form-group">
                    <label className="control-label">New email</label>
                    <input type="email" className="form-control" value={this.state.email} name="email" onChange={this.change_handler} required autoComplete="new-email"/>
                </div>
                <div className="form-group">
                    <label className="control-label">New password</label>
                    <input type="password" className="form-control" value={this.state.password} name="password" onChange={this.change_handler} required autoComplete="new-password"/>
                </div>
                <div className="form-group">
                    <label className="control-label">Confirm new password</label>
                    <input type="password" className="form-control"  value={this.state.confirm_password} name="confirm_password" onChange={this.change_handler} required autoComplete="new-password"/>
                </div>
                <input type="submit" className="btn btn-primary" value="Register"/>
            </form>
        );
    }
}