import React from 'react';
import Registration from './registration';
import Login from './login';
import api_functions from '../api';

class User extends React.Component {
    constructor() {
        super();
        this.state = {
            token : "",
            email : "",
            user_name : "",
            user_flags : []
        };
        this.set_up_user_info = this.set_up_user_info.bind(this);
        this.clear_user = this.clear_user.bind(this);
        this.return_row_property = this.return_row_property.bind(this);
    }

    set_up_user_info(new_token) {
        let options = api_functions.get_post();
        options.body = JSON.stringify({
            token : new_token
        });
        fetch(api_functions.get_api() + "/user/profile", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok) {
                this.setState({
                    token : new_token,
                    first_name : api_call.person.first_name,
                    last_name : api_call.person.last_name,
                    email : api_call.person.email,
                    user_flags : api_call.flags
                });
            } else {
                alert("Given token is not good");
            }
        })
        .catch(function(){
            alert("Could not register the account");
        });
    }

    clear_user() {
        this.setState({
            token : "",
            email : "",
            first_name : "",
            last_name : ""
        });
    }

    return_row_property(name) {
        return (
            <div>
                <dt className="col-sm-4">
                    {name.replace("_"," ")}
                </dt>
                <dd className="col-sm-10">
                    { (this.state[name] !== "" ? this.state[name]:"") }
                </dd>
            </div>
        );
    }

    render() {
        if(this.state.token === "") {
            return (
                <div className="row">
                    <Registration/>
                    <Login login_callback = { this.set_up_user_info }/>
                </div>
            );
        } else {
            return (
                <div className="col-sm-6">
                    <button className="btn btn-primary float-right" onClick={this.clear_user}>
                        Log Out
                    </button>
                    <dl className="column">
                        {this.return_row_property("email")}
                        {this.return_row_property("user_name")}
                    </dl>
                </div>
            );
        }
    }
}

export default User;