/**
 * Set authentication cookies
 *
 * @param data {Object} - The data object containing the access token and refresh token
 * @param data.accessToken {string} - The access token
 * @param data.refreshToken {string} - The refresh token
 */
function setAuthCookies(data) {
  setCookie("access_token", data.accessToken);
  setCookie("refresh_token", data.refreshToken);
}

function login(event) {
  event.preventDefault();
  const form = event.target;
  const data = {
    email: form.email.value,
    password: form.password.value,
  };
  fetch(form.action, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then(function (response) {
    if (response.ok) {
      response.json().then(function (data) {
        setAuthCookies(data);
        window.location.href = "/";
      });
    } else {
      response.json().then(function (data) {
        alert("Failed to login: " + data.errors[0]);
      });
    }
  });
}

/**
 * Register a new user
 *
 * @param event {Event} - The event object
 * @param event.target {Element} - The form element
 * @param event.target.email {string} - The email address
 * @param event.target.nickname {string} - The nickname
 */
function register(event) {
  event.preventDefault();

  const form = event.target;
  const data = {
    email: form.email.value,
    nickname: form.nickname.value,
    password: form.password.value,
  };

  fetch(form.action, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then(function (response) {
    if (response.ok) {
      window.location.href = "/login";
    } else {
      response.json().then(function (data) {
        alert(data.message);
      });
    }
  });
}

/**
 * Check if the user is authenticated
 *
 * @returns {boolean} - True if the user is authenticated, false otherwise
 */
function isAuth() {
  return document.cookie.indexOf("access_token") !== -1;
}

/**
 * Get the authentication token
 *
 * @returns {null|string} - The authentication token or null if not found
 */
function getAuth() {
  const cookies = document.cookie.split(";");
  for (let i = 0; i < cookies.length; i++) {
    const cookie = cookies[i].split("=");
    if (cookie[0].trim() === "access_token") {
      return cookie[1];
    }
  }
  return null;
}

function removeAuthCookies() {
  const cookies = document.cookie.split(";");
  for (let i = 0; i < cookies.length; i++) {
    const cookie = cookies[i].split("=");
    if (
      cookie[0].trim() === "access_token" ||
      cookie[0].trim() === "refresh_token"
    ) {
      removeCookie(cookie[0].trim());
    }
  }
}

/**
 * Logout the user
 */
function logout() {
  removeAuthCookies();
  window.location.href = "/";
}
