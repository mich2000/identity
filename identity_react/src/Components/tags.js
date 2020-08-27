import React from 'react';
import '../tag.css';

const DeleteButton = ({ onClick,value }) => {
    return (
        <li className="mb-3 mr-3 tag-li-user badge badge-pill badge-info m-2">
            {value}
            <button className="badge badge-pill badge-danger" value={value} type="button" onClick={onClick}>
                X
            </button>
        </li>
    );
}

export default class Tags extends React.Component {
    render() {
        if(this.props.list.length === 0) {
            return (<div></div>);
        }
        return (
            <ul className="list-style-none d-flex flex-wrap mb-n2">
                {(this.props.list).map((item) => (
                    <DeleteButton key={item} onClick={this.props.delete_flag_callback} value = {item}/>
                ))}
            </ul>
        );
    }
}