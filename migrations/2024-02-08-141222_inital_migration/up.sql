-- Your SQL goes here
CREATE TABLE course (
    id varchar NOT NULL,
    start_time time NOT NULL,
    end_time time NOT NULL,
    weekday varchar NOT NULL,
    subject varchar NOT NULL,
    course_type varchar NOT NULL,
    semester varchar NOT NULL,
    PRIMARY KEY (id, start_time)
);

