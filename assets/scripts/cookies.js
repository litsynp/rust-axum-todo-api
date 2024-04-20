/**
 * Get cookie by name
 *
 * @param cname {string} - The cookie name
 * @param cvalue {string} - The cookie value
 */
function setCookie(cname, cvalue) {
  document.cookie = `${cname}=${cvalue};path=/`;
}

/**
 * Remove cookie by name
 *
 * @param cname {string} - The cookie name to remove
 */
function removeCookie(cname) {
  document.cookie = cname + "=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
}
