/*
 * Scripted REST resource:
 *   HTTP method: GET
 *   Relative path: /healthcheck
 *
 * Authentication is configured on the Scripted REST API/resource with a
 * ServiceNow API-key authentication profile. The API key is not handled here.
 */
(function process(/*RESTAPIRequest*/ request, /*RESTAPIResponse*/ response) {
    response.setStatus(200);
    response.setBody(new SystemHealthcheck().getHealthcheck());
})(request, response);
