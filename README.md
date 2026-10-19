# ServiceNow Healthcheck

`sn-healthcheck` calls one or more ServiceNow endpoints, evaluates configured
health checks against the JSON response, and prints the results as a table.

## Requirements

- Rust and Cargo
- A ServiceNow endpoint that returns JSON
- An API key or other credential accepted by the endpoint

## Configuration

Copy the example environment file and set the ServiceNow values:

```bash
cp .env.example .env
```

```dotenv
CONFIG_PATH=config/config.yaml

TIMEOUT=30
TIMEOUT_INCREASE=15
DO_INCREASE_TIMEOUT_AFTER_FAIL=true
TIMEOUT_MAX_INCREASE_TIMES=5

SN_API_KEY=your-api-key
SN_INSTANCE_URL=https://your-instance.service-now.com
SN_API_URL=api/your-endpoint
SN_API_KEY_HEADER_NAME=SN-API-KEY
```

`SN_INSTANCE_URL` and `SN_API_URL` are joined with `/`. For example:

```text
https://your-instance.service-now.com/api/your-endpoint
```

The API key is sent using the header configured by
`SN_API_KEY_HEADER_NAME`.

## Check configuration

Checks are defined in `config/config.yaml`. The `instance_url`, `endpoint`,
and `api_key` fields contain environment variable names, not literal values.

```yaml
instances:
  - instance_url: SN_INSTANCE_URL
    endpoint: SN_API_URL
    api_key: SN_API_KEY
    checks:
      - name: sysevent_pending
        description: Too many events are pending
        json_path: /result/sysevent/pending_count
        warning_threshold: 100
        critical_threshold: 200
        operator: HIGHER_OR_EQUAL
        enabled: true
```

Supported operators are:

- `LOWER`
- `LOWER_OR_EQUAL`
- `EQUAL`
- `HIGHER_OR_EQUAL`
- `HIGHER`

`json_path` uses JSON Pointer syntax. The configured value must be numeric for
the check to be evaluated. A disabled check or a missing/non-numeric value is
reported as `NOT EVALUATED`.

When the configured operator matches the warning threshold, the result is
`WARNING`. When it matches the critical threshold, the result is `CRITICAL`.
Otherwise, the result is `OK`.

## Expected API response

For the example check above, the endpoint should return a response containing:

```json
{
  "result": {
    "sysevent": {
      "pending_count": 0
    }
  }
}
```

## Running

Check that the project compiles:

```bash
cargo check
```

Run the health check:

```bash
cargo run
```

The report is printed to standard output:

```text
+------------------+----------------------------+--------+
| CHECK            | DESCRIPTION                | STATUS |
+------------------+----------------------------+--------+
| sysevent_pending | Too many events are pending | OK     |
+------------------+----------------------------+--------+
```

If a request fails, the client retries according to the timeout settings and
prints an error message when all attempts fail.

## ServiceNow PDI integration

The repository includes copy/paste-ready ServiceNow scripts in
`servicenow/`:

- `script_includes/system_healtcheck.js` defines the `SystemHealthcheck`
  Script Include.
- `scripted_rest_api/healthcheck_resource.js` defines the GET resource.

### 1. Create the integration user

Create one dedicated user in **User Administration > Users**, for example:

```text
User ID: sn_healthcheck
Name: ServiceNow Healthcheck
Active: true
Web service access only: true
```

Do not use an administrator account. Give this user only the read permissions
required by the tables queried by `SystemHealthcheck`.

### 2. Create the Script Include

Open **System Definition > Script Includes**, create a new Script Include
named `SystemHealthcheck`, and paste the contents of
`servicenow/script_includes/system_healtcheck.js`.

Set **Accessible from** to the scope that owns the REST API. If the Scripted
REST API is in another scope, allow the required cross-scope access.

### 3. Create the Scripted REST API

Open **System Web Services > Scripted REST APIs** and create:

```text
Name: Healthcheck
Namespace: x_sn_healthcheck
API ID: healthcheck
```

Add a resource with:

```text
HTTP method: GET
Relative path: /healthcheck
Requires authentication: true
```

Configure the API to use ServiceNow's API-key authentication and associate the
credential with the dedicated `sn_healthcheck` user. Copy the generated key
into `SN_API_KEY`. Authentication is handled by ServiceNow before the resource
script runs; do not add custom API-key lookup code.

On the API **Security** tab, add a REST endpoint ACL:

```text
ACL type: REST_Endpoint
HTTP method: GET
Path: /healthcheck
Requires role: the role assigned to sn_healthcheck
```

Select this ACL in **Default ACLs**, and enable **Requires ACL
authorization** on the resource. The API key authenticates the user; this ACL
authorizes access to the REST resource.

Paste `servicenow/scripted_rest_api/healthcheck_resource.js` into the resource
script. The endpoint URL will be:

```text
https://<instance>.service-now.com/api/x_sn_healthcheck/healthcheck
```

Use the corresponding values in `.env`:

```dotenv
SN_INSTANCE_URL=https://<instance>.service-now.com
SN_API_URL=api/x_sn_healthcheck/healthcheck
SN_API_KEY_HEADER_NAME=SN-API-KEY
```

### 4. Test the PDI endpoint

Before running the Rust client, test the endpoint with:

```bash
curl --fail-with-body \
  -H "SN-API-KEY: $SN_API_KEY" \
  "$SN_INSTANCE_URL/$SN_API_URL"
```

The response includes metrics for pending events, open incidents, changes,
problems, requested approvals, failed scheduled jobs, and in-progress update
sets. Add checks for these metrics in `config/config.yaml` using JSON Pointer
paths such as `/result/incident/open_count` and
`/result/approval/pending_count`.
