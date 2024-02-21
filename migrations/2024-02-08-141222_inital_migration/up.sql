-- Your SQL goes here
CREATE TABLE curriculum (
    id varchar NOT NULL PRIMARY KEY,
    name_en varchar NOT NULL,
    name_de varchar NOT NULL,
    semester varchar NOT NULL
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
    faculty varchar NOT NULL,
    ects float NOT NULL,
    PRIMARY KEY (id, start_time, weekday, curriculum)
);

