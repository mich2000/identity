let api = 'http://localhost:8000';
//basic options for every api fetch call
let basic_options = function() {
    return {
        mode: "cors", // no-cors, *cors, same-origin
        cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
        credentials: "same-origin", // include, *same-origin, omit
        headers: {
            "Content-Type": "application/json",
            "Accept": "application/json"
        }
    };
}

const api_functions = {
    get_api() {
        return api;
    },
    method_get() {
        return basic_options();
    },
    method_delete() {
        let opties = basic_options();
        opties.method = "DELETE";
        return opties;
    },
    method_post() {
        let opties = basic_options();
        opties.method = "POST";
        return opties;
    },
    method_put() {        
        let opties = basic_options();
        opties.method = "PUT";
        return opties;
    }
}

export default api_functions;