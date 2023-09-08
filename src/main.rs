#[allow(unused)]
use std::collections::HashMap;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

mod read_file;
use read_file::*;


#[derive(Parser,Debug)]
struct InputFilePath{

    #[clap(long)]
    e : String,
    #[clap(long)]
    d: String,
    #[clap(long)]
    s:String,
    #[clap(long)]
    l:String,
    #[clap(long)]
    o:String,
    #[clap(long)]
    id :i32,

}

#[derive(Debug)]
pub struct Employee{
    emp_id : i32,
    emp_name : String,
    dept_id : i32,
    mobile_no : String,
    email: String,
}

#[derive(Debug)]
pub struct Dept{
    dept_id: i32,
    dept_title: String,
    dept_strength :i32,
}

#[derive(Debug)]
pub struct Salary{
    emp_id:i32,
    salary_id:i32,
    salary_date:String,
    salary:f64,
    salary_status :String,
}

#[derive(Debug)]
pub struct Leave{
    emp_id: i32,
    leave_id: i32,
    leave_from: f64,
    leave_to: f64,
    leave_type : String,
    leave_count : i32,
}

#[derive(Debug)]
struct Output{
    emp_id:i32,
    emp_name:String,
    dept_title:String,
    mobile_no:String,
    email:String,
    salary_status:String,
    on_leave:i32,
}


fn main() {

    let file_path = InputFilePath::parse();
    // let mut v : Vec<String> = vec![];
    
    let mut emp_data:HashMap<String,Employee> = HashMap::new();
    read_txt_file(file_path.e, file_path.id,&mut emp_data);

    // println!("{:?}",emp_data);
    
    let emp_id = emp_data.get(&file_path.id.to_string()).unwrap().emp_id;

    // dept file data
    let dept_id = emp_data.get(&file_path.id.to_string()).unwrap().dept_id;
    let mut dept_data :HashMap<String,Dept> = HashMap::new();
    read_dept_file_data(file_path.d,dept_id, &mut dept_data, 
        "dept");
    // println!("{:?}",dept_data);


    // salary data
    let mut salary_data :HashMap<String,Salary> = HashMap::new();
    read_salary_data(file_path.s,emp_id, &mut salary_data, 
        "salary");
    // println!("{:?}",salary_data);

    // leave data
    let mut leave_data :HashMap<String,Leave> = HashMap::new();
    read_leave_data(file_path.l,emp_id, &mut leave_data, 
        "leave");
    // println!("{:?}",leave_data);


    // let mut data_file = OpenOptions::new()
    //     .append(true)
    //     .open("output.txt")
    //     .expect("cannot open file");

    let dept_data_id = dept_id.to_string();
    let out = Output{
        emp_id : emp_id,
        emp_name : emp_data.get(&file_path.id.to_string()).unwrap().emp_name.clone(),
        dept_title : dept_data.get(&dept_data_id).unwrap().dept_title.clone(),
        mobile_no: emp_data.get(&file_path.id.to_string()).unwrap().mobile_no.clone(),
        email: emp_data.get(&file_path.id.to_string()).unwrap().email.clone(),
        salary_status: salary_data.get(&file_path.id.to_string()).unwrap().salary_status.clone(),
        on_leave: leave_data.get(&file_path.id.to_string()).unwrap().leave_count,
    };

    // println!("{:?}",out);
    // Write to a file
    let mut content = String::new();

    let heading = format!("Emp_id#Emp_name#dept_title#mobile_number#email#salary_status#on_leave\n");
    content.push_str(&heading);
    let file_data = format!("{:?}#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}", out.emp_id,out.emp_name,
        out.dept_title,out.mobile_no,out.email,out.salary_status,out.on_leave);

    content.push_str(&file_data);
    
    std::fs::write("output.txt", content);
    
    
}
