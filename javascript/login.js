// create references to our HTML to get data from the form inputs
const loginButton = document.getElementById('loginButton')
const username = document.getElementById('defaultLoginFormUsername');
const password = document.getElementById('defaultLoginFormPassword');
const message = document.getElementById("loginMessage");

// add an event listener to the login button, which sends the data from the form to the login endpoint
loginButton.addEventListener('click', () => {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/auth/login", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    // set a handler for the readystatechange event
    xhr.onreadystatechange = function () {
        // check if the response is ready
        if (xhr.readyState === 4) {
            // chech if the response is ok
            if (xhr.status === 200) {
                // get the token from the header of the response
                let token = xhr.getResponseHeader("token");
                // store the token in localstorage
                localStorage.setItem("user-token", token);
                // redirect the page
                window.location.replace(document.location.origin)
            } else {
                message.innerText = "login failed please try again";
            }
        }
    };

    let data = JSON.stringify(
        {
            "user_name": username.value,
            "password": password.value
        }
    );
    xhr.send(data);
    message.innerText = "logging in";
})