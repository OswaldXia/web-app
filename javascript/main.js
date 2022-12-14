
// stops a user without a token viewing the main items view
if (localStorage.getItem("user-token") == null) {
    window.location.replace(document.location.origin + "/login");
} else {
    let item_cache_date = Date.parse(localStorage.getItem("item-cache-date"));
    let now = new Date();
    let difference = Math.round(
        (now - item_cache_date) / 1000
    );

    if (difference <= 120) {
        let response_text = localStorage.getItem("item-cache-data");
        runRenderProcess(response_text);
    } else {
        getItems();
    }
}

let button = document.getElementById("create-button")
button.addEventListener("click", createItem);


/**
 * Renders the to do items from the backend into a HTML div.
 *
 * @param items {Array} - list of to do items
 * @param processType {String} - the type of process that the button belonging to the to do item
 * @param elementId {String} - the id of the HTML element that the items will be inserted
 * @param processFunction {editItem | deleteItem} - function that is fired once the button is clicked
 */
function renderItems(items, processType, elementId, processFunction) {
    // define the string containing the HTML being inserted under the placeholder variable.
    let placeholder = "<div>";
    // define an empty array called itemsMeta, where we can place titles in order to add even listeners later on
    let itemsMeta = [];

    for (i = 0; i < items.length; i++) {
        let title = items[i]["title"];
        let placeholderId = processType + "-" + title.replaceAll(" ", "-");

        placeholder += '<div class="itemContainer">' +
            '<p>' + title + '</p>' +
            '<div class="actionButton" ' +
            'id="' + placeholderId + '">'
            + processType +
            '</div>' + "</div>";

        itemsMeta.push({
            "id": placeholderId,
            "title": title
        });
    }
    placeholder += "</div>"
    document.getElementById(elementId).innerHTML = placeholder;

    for (i = 0; i < itemsMeta.length; i++) {
        document.getElementById(itemsMeta[i]["id"])
            .addEventListener("click", processFunction);
    }
}

function runRenderProcess(response_text) {
    let data = JSON.parse(response_text);
    renderItems(
        data["pending_items"], "edit", "pendingItems", editItem);
    renderItems(
        data["done_items"], "delete", "doneItems", deleteItem);
    document.getElementById("completeNum").innerHTML = data["done_item_count"];
    document.getElementById("pendingNum").innerHTML = data["pending_item_count"];
}

/**
 * Packages an API call ready to be sent.
 *
 * @param url {String} - the URL endpoint for the API call
 * @param method {String} - the method of the API call => POST, GET, PUT
 * @returns {XMLHttpRequest} - the API packaged API request
 */
function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener("readystatechange", function () {
        if (this.readyState === this.DONE) {
            //  redirect the user to log in if the API call is unauthorized
            if (this.status == 401) {
                window.location.replace(document.location.origin + "/login/");
            } else {
                runRenderProcess(this.responseText);
                localStorage.setItem(
                    "item-cache-date", new Date()
                );
                localStorage.setItem(
                    "item-cache-data", this.responseText
                );
            }
        }
    });
    xhr.open(method, "/api/v1" + url);
    xhr.setRequestHeader("content-type", "application/json");
    // get the token from storage and insert it into the header for the request to send it
    xhr.setRequestHeader("user-token", localStorage.getItem("user-token"));
    return xhr
}

/**
 * Gets the title from this, and calls the edit API endpoint.
 */
function editItem() {
    let title = this.id.replaceAll("-", " ").replace("edit ", "");
    let call = apiCall("/item/edit", "PUT");
    let json = {
        "title": title,
        "status": "done"
    };
    call.send(JSON.stringify(json));
}

/**
 * Gets the title from this, and calls the delete API endpoint.
 */
function deleteItem() {
    let title = this.id.replaceAll("-", " ").replace("delete ", "");
    let call = apiCall("/item/delete", "POST");
    let json = {
        "title": title,
        "status": "done"
    };
    call.send(JSON.stringify(json));
}

/**
 * Calls the get items API.
 */
function getItems() {
    let call = apiCall("/item/get", 'GET');
    call.send()
}

/**
 * Gets the title from the HTML with "name" as ID, and calls the create API endpoint with it.
 */
function createItem() {
    let title = document.getElementById("name");
    let call = apiCall("/item/create/" + title.value, "POST");
    call.send();
    document.getElementById("name").value = null;
}