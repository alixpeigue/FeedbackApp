CREATE TABLE upvote (
    worker_id INT REFERENCES worker(id) NOT NULL,
    report_id INT REFERENCES report(id) NOT NULL,
    PRIMARY KEY (worker_id, report_id)
);
