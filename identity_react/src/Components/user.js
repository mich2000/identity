import React from 'react';
import Registration from './registration';
import Login from './login';
import Input from './input';
import Tags from './tags';
import api_functions from '../api';

export default class User extends React.Component {
    constructor() {
        super();
        this.state = {
            token : "",
            email : "",
            user_name : "",
            user_flags : [],
            error : ""
        };
        this.set_up_user_info = this.set_up_user_info.bind(this);
        this.clear_user = this.clear_user.bind(this);
        this.return_row_property = this.return_row_property.bind(this);
        this.log_error = this.log_error.bind(this);
        this.add_flag = this.add_flag.bind(this);
        this.remove_flag = this.remove_flag.bind(this);
    }
    
    log_error(new_messsage) {
        this.setState({
            error : new_messsage
        });
    }

    set_up_user_info(new_token) {
        let options = api_functions.method_get();
        options.headers["X-API-Key"] = new_token;
        options.body = null;
        fetch(api_functions.get_api() + "/user/profile", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok) {
                this.setState({
                    token : new_token,
                    first_name : api_call.person.first_name,
                    last_name : api_call.person.last_name,
                    email : api_call.person.email,
                    user_flags : this.state.user_flags.concat(api_call.person.flags),
                    error : ""
                });
            } else {
                this.log_error(api_call.error);
            }
        })
        .catch((e) => {
            this.log_error(e.message);
        });
    }

    add_flag(input_event, new_flag) {
        let options = api_functions.method_put();
        options.headers["X-API-Key"] = this.state.token.toString();
        options.body = JSON.stringify({
            flag : new_flag
        });
        fetch(api_functions.get_api() + "/user/flag/add", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok && !this.state.user_flags.includes(new_flag)) {
                this.setState({ user_flags : this.state.user_flags.concat(new_flag) })
            }
        })
        .catch((e) => {
            this.log_error(e.message);
        });
        input_event.preventDefault();
        input_event.stopPropagation();
    }

    remove_flag(input_event) {
        let options = api_functions.method_delete();
        options.headers["X-API-Key"] = this.state.token.toString();
        const value = input_event.target.value;
        options.body = JSON.stringify({
            flag : value
        });
        fetch(api_functions.get_api() + "/user/flag/remove", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok && this.state.user_flags.includes(value)) {
                let flags = this.state.user_flags;
                flags.splice(flags.indexOf(value),1);
                this.setState({ user_flags : flags })
            }
        })
        .catch((e) => {
            this.log_error(e.message);
        });
        input_event.preventDefault();
        input_event.stopPropagation();
    }

    change_email(token, new_email) {

    }

    clear_user() {
        this.setState({
            token : "",
            email : "",
            first_name : "",
            last_name : "",
            user_flags : []
        });
    }

    return_row_property(name) {
        return (
            <div>
                <dt>
                    {name.replace("_"," ")}
                </dt>
                <dd>
                    {(this.state[name] !== "" ? this.state[name]:"")}
                </dd>
            </div>
        );
    }

    render() {
        if(this.state.token === "") {
            return (
                <div>
                    <span className="font-weight-bold text-danger">{this.state.error}</span>
                    <div>
                        <Registration login_callback = { this.set_up_user_info } log_error= {this.log_error}/>
                        <Login login_callback = { this.set_up_user_info } log_error= {this.log_error}/>
                    </div>
                </div>
            );
        }
        return (
            <div className="col-sm-6">
                <div className="div-inline-block float-right">
                    <button className="btn btn-primary float-right" onClick={this.clear_user}>
                        Log Out
                    </button>
                    <span className="font-weight-bold text-danger">{this.state.error}</span>
                </div>
                <dl className="column">
                    {this.return_row_property("email")}
                    {this.return_row_property("user_name")}
                </dl>
                <h2>Flags</h2>
                <Input input_callback = {(input_event, new_flag) => this.add_flag(input_event, new_flag)} name = "Add flag"/>
                <Tags list={(this.state.user_flags || [])} delete_flag_callback={(e) => this.remove_flag(e)}/>
            </div>
        );
    }
}