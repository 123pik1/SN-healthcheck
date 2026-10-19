var SystemHealthcheck = Class.create();
SystemHealthcheck.prototype = {
    initialize: function() {},

    getHealthcheck: function() {
        return {
            result: {
                generated_at: new GlideDateTime().getDisplayValue(),
                sysevent: {
                    pending_count: this._count('sysevent', 'state', 'ready')
                },
                incident: {
                    open_count: this._count('incident', 'active', true)
                },
                change: {
                    open_count: this._count('change_request', 'active', true)
                },
                problem: {
                    open_count: this._count('problem', 'active', true)
                },
                approval: {
                    pending_count: this._count('sysapproval_approver', 'state', 'requested')
                },
                scheduled_job: {
                    failed_count: this._count('sys_trigger', 'state', 'error')
                },
                update_set: {
                    in_progress_count: this._count('sys_update_set', 'state', 'in progress')
                }
            }
        };
    },

    _count: function(tableName, fieldName, fieldValue) {
        var aggregate = new GlideAggregate(tableName);
        if (!aggregate.isValid() || !aggregate.isValidField(fieldName)) {
            return 0;
        }

        aggregate.addQuery(fieldName, fieldValue);
        aggregate.addAggregate('COUNT');
        aggregate.query();

        if (aggregate.next()) {
            return parseInt(aggregate.getAggregate('COUNT'), 10);
        }

        return 0;
    },

    type: 'SystemHealthcheck'
};