CREATE TABLE structures_usage (
    package_id         CHAR(66) NOT NULL,
    network            VARCHAR(10) NOT NULL,
    module_name        TEXT NOT NULL,
    datatype_name      TEXT NOT NULL,
    target_module_name TEXT NOT NULL,
    call_chain         TEXT[] NOT NULL DEFAULT '{}',
    function_name      TEXT NOT NULL,
    address_owned      JSONB,
    object_owned       JSONB,
    wrapped            JSONB,
    shared             JSONB,
    immutable          JSONB,
    emitted            JSONB,

    PRIMARY KEY(package_id, network, module_name, datatype_name, target_module_name, function_name),
    FOREIGN KEY(package_id, network)
        REFERENCES objects(object_id, network)
        ON DELETE CASCADE,
    FOREIGN KEY(package_id, network, module_name)
        REFERENCES package_modules(package_id, network, module_name)
        ON DELETE CASCADE,
    FOREIGN KEY(package_id, network, module_name, datatype_name)
        REFERENCES structures(package_id, network, module_name, datatype_name)
        ON DELETE CASCADE,
    FOREIGN KEY(package_id, network, target_module_name, function_name)
        REFERENCES functions(package_id, network, module_name, function_name)
        ON DELETE CASCADE
);