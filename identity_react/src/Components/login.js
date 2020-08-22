import React from 'react';
import api_functions from '../api';

class Login extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            email : "",
            password : ""
        };
        this.change_handler = this.change_handler.bind(this);
        this.login = this.login.bind(this);
     }
    
    login(e) {
        let opties = api_functions.get_post();
        opties.body = JSON.stringify({
            email : this.state.email,
            password : this.state.password
        });
        fetch(api_functions.get_api() + "/user/login", opties)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok) {
                this.props.login_callback(api_call.token);
            }
        })
        .catch(() => {
            alert("Could not register the account");
        });
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
            <div className="col-md-3">
                <h2>Login</h2>
                <div className="form-group">
                    <label className="control-label">New email</label>
                    <input type="email" className="form-control" value={this.state.email} name="email" onChange={this.change_handler}/>
                </div>
                <div className="form-group">
                    <label className="control-label">New password</label>
                    <input type="password" className="form-control" value={this.state.password} name="password" onChange={this.change_handler}/>
                </div>
                <input type="submit" className="btn btn-primary" value="Login" onClick={(e) => this.login(e)} />
            </div>
);
    }
}

export default Login;