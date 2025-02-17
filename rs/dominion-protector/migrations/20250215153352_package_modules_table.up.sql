CREATE TABLE package_modules (
    package_id      CHAR(66) NOT NULL,
    network         VARCHAR(10) NOT NULL,
    module_name     TEXT NOT NULL,
    module_bytecode BYTEA NOT NULL,

    PRIMARY KEY(package_id, network, module_name),
    FOREIGN KEY(package_id, network)
        REFERENCES objects(object_id, network)
        ON DELETE CASCADE
);
