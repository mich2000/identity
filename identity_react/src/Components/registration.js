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
        if(this.state.confirm_password == this.state.password) {
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
            .then((e) => e.json())
            .then((e) => {
                result.ok = e.Status === "OK"
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
        let nam = event.target.name;
        let val = event.target.value;
        this.setState({[nam] : val});
    }

    render() {
        return (
            <div>
                <label>New email</label>
                <input type="email" value={this.state.email} name="email"
                onChange= { this.change_handler }/>
                <label>New password</label>
                <input type="password" value={this.state.password} name="password"
                onChange= { this.change_handler }/>
                <label>Confirm new password</label>
                <input type="password" value={this.state.confirm_password} name="confirm_password"
                onChange= { this.change_handler }/>
                <input type="submit" value="Register" onClick={(e) => this.registration(e)} />
            </div>
        );
    }
}

export default Registration;