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
            let opties = api_functions.get_post();
            opties.body = JSON.stringify({
                email : this.state.email,
                password : this.state.password,
                confirm_password : this.state.password
            });
            let result = {
                ok : false
            };
            fetch(api_functions.get_api() + "/user/registration", opties)
            .then((api_call) => api_call.json())
            .then((api_call) => {
                result.ok = api_call.ok
            })
            .catch(function(){
                alert("Could not register the account");
            });
            if(result.ok) {
                alert("Registration has been succesfull.");
            } else {
                alert("Registration has been unsuccesfull.");
            }
        } else {
            alert("Password and confirm password aren't the same.");
        }
        e.preventDefault();
        e.stopPropagation();
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    render() {
        return (
            <form className="col-md-3" onSubmit={(e) => this.registration(e)}>
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