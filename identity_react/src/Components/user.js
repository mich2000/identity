import React from 'react';
import Registration from './registration';

class User extends React.Component {
    constructor() {
        super();
        this.state = {
            token : "",
            email : "",
            first_name : "",
            last_name : ""
        };
    }

    clear_user() {
        this.setState({
            token : "",
            email : "",
            first_name : "",
            last_name : ""
        });
    }

    render() {
        return (
            <Registration/>
        );
    }
}

export default User;