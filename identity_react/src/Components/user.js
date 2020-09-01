import React from 'react';
import Input from './input';
import DoubleInput from './double_input';
import Tags from './tags';
import api_functions from '../api';
import email from '../email';

export default class User extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            email : "",
            user_name : "",
            user_flags : [],
            error : ""
        };
        this.timer = null;
        this.set_up_user_info = this.set_up_user_info.bind(this);
        this.return_row_property = this.return_row_property.bind(this);
        this.log_error = this.log_error.bind(this);
        this.add_flag = this.add_flag.bind(this);
        this.remove_flag = this.remove_flag.bind(this);
        this.return_token = this.return_token.bind(this);
        this.update_user_name = this.update_user_name.bind(this);
        this.update_email = this.update_email.bind(this);
        this.update_password = this.update_password.bind(this);
    }
    
    log_error(new_messsage) {
        this.setState({
            error : new_messsage
        });
    }

    set_up_user_info() {
        fetch(api_functions.get_api() + "/user/profile", api_functions.put_key(api_functions.method_get(),this.return_token()))
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok) {
                this.setState({
                    user_name : api_call.person.user_name,
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

    return_token() {
        return this.props.get_token();
    }

    componentWillUnmount() {
        console.log("User has been unmounted");
        clearInterval(this.timer);
    }

    componentDidMount() {
        this.set_up_user_info();
        this.timer = setInterval(() => {
            if(this.props.get_token() !== "") {
                this.props.update_token();
            }
        }, (60 * 1000 * 9));
    }

    add_flag(value) {
        let options = api_functions.put_key(api_functions.method_put(),this.return_token());
        options.body = JSON.stringify({
            flag : value
        });
        fetch(api_functions.get_api() + "/user/flag/add", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok && !this.state.user_flags.includes(value)) {
                this.setState({ user_flags : this.state.user_flags.concat(value) })
            }
        }).catch((e) => this.log_error(e.message));
    }

    remove_flag(input_event) {
        const value = input_event.target.value;
        let options = api_functions.put_key(api_functions.method_delete(),this.return_token());
        options.body = JSON.stringify({
            flag : value
        });
        fetch(api_functions.get_api() + "/user/flag/remove", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok && this.state.user_flags.includes(value)) {
                let flags = this.state.user_flags;
                flags.splice(flags.indexOf(value),1);
                this.setState({ user_flags : flags });
            }
        })
        .catch((e) => this.log_error(e.message));
        input_event.preventDefault();
        input_event.stopPropagation();
    }

    update_user_name(value) {
        let options = api_functions.put_key(api_functions.method_put(),this.return_token());
        options.body = JSON.stringify({
            new_user_name : value
        });
        fetch(api_functions.get_api() + "/user/update", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok && !this.state.user_flags.includes(value)) {
                this.setState({user_name : value});
            }
        }).catch((e) => this.log_error(e.message));
    }

    update_email(value) {
        if(email.control_email(value)) {
            let options = api_functions.put_key(api_functions.method_put(),this.return_token());
            options.body = JSON.stringify({
                new_email : value
            });
            fetch(api_functions.get_api() + "/user/update", options)
            .then((api_call) => api_call.json())
            .then((api_call) => {
                if(api_call.ok && !this.state.user_flags.includes(value)) {
                    this.setState({email : value});
                }
            }).catch((e) => this.log_error(e.message));
        } else {
            this.log_error("Email is not in the right format.");
        }
    }

    update_password(password, confirm_password) {
        if(password === confirm_password) {
            let options = api_functions.put_key(api_functions.method_put(),this.return_token());
            options.body = JSON.stringify({
                password : password,
                confirm_password : confirm_password
            });
            fetch(api_functions.get_api() + "/user/password", options)
            .then((api_call) => api_call.json())
            .then((api_call) => {
                if(api_call.ok) {
                    alert("Password has been changed");
                } else {
                    this.log_error(api_call.error);
                }
            }).catch((e) => this.log_error(e.message));
        } else {
            this.log_error("Password and its confirmation aren't equal.");
        }
    }

    return_row_property(name) {
        return (
            <div>
                <dt>{name.replace("_"," ")}</dt>
                <dd>{(this.state[name.toString().toLowerCase()] !== "" ? this.state[name.toString().toLowerCase()]:"")}</dd>
            </div>
        );
    }

    render() {
        return (
            <div className="col-sm-10">
                <div className="div-inline-block">
                    <button className="btn btn-primary" onClick={this.props.logout}>
                        Log Out
                    </button>
                    <span className="font-weight-bold text-danger">{this.state.error}</span>
                </div>
                <div id="accordion" className="col-sm-8 m-3">
                    <div className="card">
                        <div className="card-header" id="UserInfo">
                            <h4 className="mb-0">
                                <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserInfo" aria-expanded="true" aria-controls="collapseUserInfo">User information</button>
                            </h4>
                        </div>
                        <div id="collapseUserInfo" className="collapse show" aria-labelledby="UserInfo" data-parent="#accordion">
                            <div className="card-body">
                                <dl className="column">
                                    {this.return_row_property("Email")}
                                    <Input input_callback = {(e) => this.update_email(e)} valuePlaceholder="New email" name = "Update email"/>
                                    {this.return_row_property("user_name")}
                                    <Input input_callback = {(e) => this.update_user_name(e)} valuePlaceholder="New username" name = "Update username"/>
                                </dl>
                                <a className="btn btn-primary" data-toggle="collapse" href="#collapseChangePwd" role="button" aria-expanded="false" aria-controls="collapseChangePwd">
                                    Change password
                                </a>
                                <div id="collapseChangePwd" className="collapse p-2">
                                    <DoubleInput firstPlaceholder="New password" input_callback={(value1,value2) => this.update_password(value1,value2)} secondPlaceholder="Confirm new password" hideInput="" name="Update password"/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="card">
                        <div className="card-header" id="UserFlags">
                            <h4 className="mb-0">
                                <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserFlags" aria-expanded="true" aria-controls="collapseUserFlags">Flags</button>
                            </h4>
                        </div>
                        <div id="collapseUserFlags" className="collapse show" aria-labelledby="UserFlags" data-parent="#accordion">
                            <div className="card-body">
                                <Input valuePlaceholder="New unique flag" input_callback = {(e) => this.add_flag(e)} name = "Add flag"/>
                                <Tags list={(this.state.user_flags || [])} delete_flag_callback={(e) => this.remove_flag(e)}/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}