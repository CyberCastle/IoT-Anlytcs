CREATE TABLE IF NOT EXISTS sensor_info (
    id BIGSERIAL NOT NULL,
    sensor_name VARCHAR(64) NOT NULL,
    sensor_type INTEGER NOT NULL,
    register_date DATE NOT NULL,
    is_enabled BOOLEAN NOT NULL,
    sensor_description VARCHAR(512) NOT NULL,
    measurement_unit INTEGER,
    random_serial DOUBLE PRECISION,
    last_check_status INTEGER,
    last_check_status_date TIMESTAMP,
    PRIMARY KEY(Id)
);
