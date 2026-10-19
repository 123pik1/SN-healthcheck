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
    // there would be two result layers in answer if not this .result
    response.setBody(new SystemHealthcheck().getHealthcheck().result);
})(request, response);
