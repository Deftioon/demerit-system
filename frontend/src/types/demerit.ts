export interface DemeritRecord {
  id: number;
  category: string;
  points: number;
  date_issued: string;
}

export interface TeacherRecord extends DemeritRecord {
  student_name: string;
}

export interface StudentRecord extends DemeritRecord {
  teacher_name: string;
}

export interface ParentRecord extends DemeritRecord {
  student_name: string;
  teacher_name: string;
}

export interface ChildRecord {
  id: number;
  name: string;
}

export interface AdminUserRecord {
  user_id: number;
  username: string;
  email: string;
  user_type: string;
  first_name: string;
  last_name: string;
  total_demerits: number;
  created_at: string;
  grade_level?: number | null; // Note the optional marker
  class_section?: string | null; // Note the optional marker
  children?: ChildRecord[];
}

export type DataRecord = TeacherRecord | StudentRecord | ParentRecord;
