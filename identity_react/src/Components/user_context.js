import React from 'react';
import User from './user';
import api_functions from '../api';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import UnauthenticatedHome, {About} from './normal_home';

export default class UserContext extends React.Component {
    constructor() {
        super();
        this.state = {
            token : ""
        }
        this.set_token = this.set_token.bind(this);
        this.clear_token = this.clear_token.bind(this);
        this.give_token = this.give_token.bind(this);
        this.update_token = this.update_token.bind(this);
    }

    update_token() {
        let options = api_functions.method_get();
        options.headers["X-API-Key"] = this.give_token();
        options.body = null;
        fetch(api_functions.get_api() + "/user/token", options)
        .then((api_call) => api_call.json())
        .then((api_call) => {
            if(api_call.ok) {
                this.set_token(api_call.token);
            } else {
                console.log(api_call.error);
            }
        })
        .catch((e) => {
            console.log(e.message);
        });
}

    set_token(new_token) {
        this.setState({ token : new_token });
    }

    clear_token() {
        this.setState({ token : "" });
    }

    give_token() {
        return this.state.token.toString();
    }
    
    render() {
        if(this.state.token.toString() === "") {
            return (
                <Router>
                    <div className="col-sm-10">
                        <nav className="navbar navbar-expand-lg navbar-light bg-light">
                            <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                                <span className="navbar-toggler-icon"></span>
                            </button>
                            <div className="collapse navbar-collapse" id="navbarNav">
                                <ul className="navbar-nav">
                                    <li className="nav-item">
                                        <Link className="nav-link" to="/">Home</Link>
                                    </li>
                                    <li className="nav-item">
                                        <Link className="nav-link" to="/about">About</Link>
                                    </li>
                                </ul>
                            </div>
                        </nav>
                        <Switch>
                            <Route path="/about">
                                <About />
                            </Route>
                            <Route path="/">
                                <UnauthenticatedHome login_callback={ this.set_token }/>
                            </Route>
                        </Switch>
                    </div>
                    <span className="font-weight-bold text-danger">{this.state.error}</span>
                </Router>
            );
        }
        return (
            <Router>
                <div className="col-sm-10">
                    <nav className="navbar navbar-expand-lg navbar-light bg-light">
                        <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                            <span className="navbar-toggler-icon"></span>
                        </button>
                        <div className="collapse navbar-collapse" id="navbarNav">
                            <ul className="navbar-nav">
                                <li className="nav-item">
                                    <Link className="nav-link" to="/">Home</Link>
                                </li>
                                <li className="nav-item">
                                    <Link className="nav-link" to="/about">About</Link>
                                </li>
                            </ul>
                        </div>
                    </nav>
                </div>
                <Switch>
                    <Route path="/about">
                        <About />
                    </Route>
                    <Route path="/">
                        <User get_token={this.give_token} logout={this.clear_token} update_token={this.update_token}/>
                    </Route>
                </Switch>
                <span className="font-weight-bold text-danger">{this.state.error}</span>
            </Router>
        );
    }
}