let api = 'http://localhost:8000';
//basic options for every api fetch call
let opties = {
    method: "POST",// *GET, POST, PUT, DELETE, etc.
    mode: "cors", // no-cors, *cors, same-origin
    cache: "default", // *default, no-cache, reload, force-cache, only-if-cached
    credentials: "same-origin", // include, *same-origin, omit
    headers: {
        "Content-Type": "application/json",
        "Accept": "application/json"
    }
};

const api_functions = {
    get_api() {
        return api;
    },
    get_delete() {
        opties.method = "Delete";
        return opties;
    },
    get_post() {
        opties.method = "POST";
        return opties;
    },
    get_put() {        
        opties.method = "PUT";
        return opties;
    }
}

export default api_functions;