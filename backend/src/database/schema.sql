-- Users table (for all types of users)
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    user_type TEXT NOT NULL CHECK (
        user_type IN ('admin', 'teacher', 'student', 'parent')
    ),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Students table (additional student-specific info)
CREATE TABLE students (
    student_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER UNIQUE,
    grade_level INTEGER NOT NULL,
    class_section TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);

-- Teachers table (additional teacher-specific info)
CREATE TABLE teachers (
    teacher_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER UNIQUE,
    subject TEXT NOT NULL,
    department TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);

-- Parents table (if not already present)
CREATE TABLE parents (
    parent_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER UNIQUE,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);

CREATE TABLE parent_student (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id INTEGER,
    student_id INTEGER,
    FOREIGN KEY (parent_id) REFERENCES parents (parent_id),
    FOREIGN KEY (student_id) REFERENCES students (student_id)
);

-- Demerit categories
CREATE TABLE demerit_categories (
    category_id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT NOT NULL,
    description TEXT,
    default_points INTEGER NOT NULL
);

-- Demerit records
CREATE TABLE demerit_records (
    demerit_id INTEGER PRIMARY KEY AUTOINCREMENT,
    student_id INTEGER NOT NULL,
    teacher_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    points INTEGER NOT NULL,
    description TEXT,
    date_issued TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students (student_id),
    FOREIGN KEY (teacher_id) REFERENCES teachers (teacher_id),
    FOREIGN KEY (category_id) REFERENCES demerit_categories (category_id)
);

-- Example data for demerit categories
INSERT INTO
    demerit_categories (category_name, description, default_points)
VALUES
    (
        'Late to Class',
        'Student arrived late to class without valid reason',
        1
    ),
    (
        'Misconduct',
        'Inappropriate behavior during class',
        2
    ),
    (
        'Incomplete Homework',
        'Failed to complete assigned homework',
        1
    ),
    (
        'Dress Code Violation',
        'Not following school dress code',
        1
    ),
    (
        'Disruptive Behavior',
        'Causing disruption during school activities',
        3
    );
