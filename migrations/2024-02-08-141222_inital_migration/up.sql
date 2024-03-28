-- Your SQL goes here
CREATE TYPE Processing_Error AS enum (
    'None',
    'MissingVariants',
    'MissingDescription',
    'MissingAppointments',
    'MissingOrganization'
);

CREATE TABLE curriculum (
    id varchar NOT NULL PRIMARY KEY,
    name_en varchar NOT NULL,
    name_de varchar NOT NULL,
    semester varchar NOT NULL
);

INSERT INTO curriculum
    VALUES ('0000', 'NO CURR', 'NO CURR', '000');

CREATE TABLE organization (
    id varchar NOT NULL PRIMARY KEY,
    name varchar NOT NULL,
    parent varchar NOT NULL,
    kind varchar NOT NULL
);

-- to keep track of already processed courses
CREATE TABLE course (
    id varchar NOT NULL PRIMARY KEY,
    course_type varchar NOT NULL,
    sws float NOT NULL,
    name_en varchar NOT NULL,
    name_de varchar NOT NULL,
    semester varchar NOT NULL,
    processing_error Processing_Error NOT NULL
);

CREATE TABLE lecture (
    id varchar NOT NULL,
    start_time time NOT NULL,
    end_time time NOT NULL,
    weekday varchar NOT NULL,
    subject varchar NOT NULL,
    course_type varchar NOT NULL,
    name_en varchar NOT NULL,
    name_de varchar NOT NULL,
    semester varchar NOT NULL,
    curriculum varchar NOT NULL REFERENCES curriculum (id),
    description varchar NOT NULL,
    organization varchar NOT NULL REFERENCES organization (id),
    ects float NOT NULL,
    PRIMARY KEY (id, start_time, weekday, curriculum)
);

