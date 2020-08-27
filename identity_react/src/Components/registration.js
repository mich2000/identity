import React from 'react';
import api_functions from '../api';

class Registration extends React.Component {
    constructor() {
        super();
        this.state = {
            email : "",
            password : "",
            confirm_password : ""
        };
        this.change_handler = this.change_handler.bind(this);
        this.registration = this.registration.bind(this);
     }
    
     registration(e) {
        if(this.state.confirm_password === this.state.password) {
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
                    this.props.log_error("");
                    alert("Registration has succeeded.");
                } else {
                    this.props.log_error(api_call.error);
                }
            })
            .catch(function(){
                this.props.log_error("Could not register the account");
            });
        } else {
            this.props.log_error("Password and confirm password aren't the same.");
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
                    <input type="new-email" className="form-control" value={this.state.email} name="email" onChange={this.change_handler}/>
                </div>
                <div className="form-group">
                    <label className="control-label">New password</label>
                    <input type="new-password" className="form-control" value={this.state.password} name="password" onChange={this.change_handler}/>
                </div>
                <div className="form-group">
                    <label className="control-label">Confirm new password</label>
                    <input type="new-password" className="form-control"  value={this.state.confirm_password} name="confirm_password" onChange={this.change_handler}/>
                </div>
                <input type="submit" className="btn btn-primary" value="Register"/>
            </form>
        );
    }
}

export default Registration;