CREATE TYPE Visibility AS ENUM ('Private', 'Public', 'Package');

CREATE TABLE functions (
    package_id          CHAR(66) NOT NULL,
    network             VARCHAR(10) NOT NULL,
    module_name         TEXT NOT NULL,
    function_name       TEXT NOT NULL,
    visibility          Visibility NOT NULL,
    is_entry            BOOLEAN NOT NULL,
    is_initializer      BOOLEAN NOT NULL,
    type_argument_count INTEGER NOT NULL,
    parameter_count     INTEGER NOT NULL,
    return_count        INTEGER NOT NULL,
    source_code         TEXT,

    PRIMARY KEY(package_id, network, module_name, function_name),
    FOREIGN KEY(package_id, network)
        REFERENCES objects(object_id, network)
        ON DELETE CASCADE,
    FOREIGN KEY(package_id, network, module_name)
        REFERENCES package_modules(package_id, network, module_name)
        ON DELETE CASCADE
);
