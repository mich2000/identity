import React from 'react';
import User from './user';
import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link
} from "react-router-dom";

  function Home() {
    return (
        <div>
            <h1>Rust React identity application</h1>
            <User/>
        </div>
    );
}

function About() {
    return (
        <div>
            <h1>About page of the website</h1>
            <p>
                This react application has been made to demonstrate the rust web api application this will also give the possibility to register and login users. You can add and delete unique flags.
            </p>
        </div>
    );
}

class Context extends React.Component {
    render() {
        return (
            <Router>
                <div>
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
                        <Home />
                    </Route>
                </Switch>
                </div>
            </Router>
        );
    }
}

export default Context;