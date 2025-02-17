CREATE TABLE structures (
    package_id          CHAR(66) NOT NULL,
    network             VARCHAR(10) NOT NULL,
    module_name         TEXT NOT NULL,
    datatype_name       TEXT NOT NULL,
    origin              CHAR(66) NOT NULL,
    field_count         INTEGER NOT NULL,
    type_argument_count INTEGER NOT NULL,
    source_code         TEXT,
    has_key             BOOLEAN NOT NULL,
    has_copy            BOOLEAN NOT NULL,
    has_drop            BOOLEAN NOT NULL,
    has_store           BOOLEAN NOT NULL,

    PRIMARY KEY(package_id, network, module_name, datatype_name),
    FOREIGN KEY(package_id, network)
        REFERENCES objects(object_id, network)
        ON DELETE CASCADE,
    FOREIGN KEY(package_id, network, module_name)
        REFERENCES package_modules(package_id, network, module_name)
        ON DELETE CASCADE
);
