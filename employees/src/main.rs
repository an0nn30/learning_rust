mod menus {
    use std::process::Command;

    pub struct Menu {
        title: String,
        options: Vec<String>,
    }
    impl Menu {
        pub fn new(title: String, options: Vec<String>) -> Menu {
            Menu { title, options }
        }
        pub fn show(&self) {
            Command::new("clear")
                .status()
                .expect("Failed to clear screen");
            println!("{}", self.title);
            for (i, option) in self.options.iter().enumerate() {
                println!("{}. {}", i + 1, option);
            }
        }
        pub fn get_option(&self) -> usize {
            let mut option = String::new();
            std::io::stdin()
                .read_line(&mut option)
                .expect("Failed to read line");
            let option: usize = option.trim().parse().expect("Please type a number!");
            option
        }
        pub fn wait(&self) {
            println!("Press enter to continue...");
            let mut option = String::new();
            std::io::stdin()
                .read_line(&mut option)
                .expect("Failed to read line");
        }
    }
}

mod employee {
    #[derive(Clone)]
    pub struct Job {
        pub title: String,
        pub salary: u32,
        pub job_type: JobType,
    }
    #[derive(Clone)]
    pub enum JobType {
        FullTime,
        PartTime,
    }
    #[derive(Clone)]
    pub struct Employee {
        name: String,
        age: u8,
        job: Job,
    }
    impl Employee {
        pub fn new(name: String, age: u8, job: Job) -> Employee {
            Employee { name, age, job }
        }
        pub fn get_name(&self) -> &str {
            &self.name
        }
        pub fn get_age(&self) -> u8 {
            self.age
        }
        pub fn get_job(&self) -> &Job {
            &self.job
        }
    }
}

mod company {
    use std::collections::HashMap;

    use crate::employee::Employee;
    pub struct Company {
        name: String,
        employees: Vec<Employee>,
        departments: HashMap<String, Vec<Employee>>,
    }
    impl Company {
        pub fn new(name: String) -> Company {
            Company {
                name,
                employees: Vec::new(),
                departments: HashMap::new(),
            }
        }
        pub fn add_employee(&mut self, employee: Employee) {
            self.employees.push(employee);
        }
        pub fn get_employees(&self) -> &Vec<Employee> {
            &self.employees
        }
        pub fn add_new_department(&mut self, department: String) {
            self.departments.insert(department, Vec::new());
        }
        pub fn get_employees_by_department(&self, department: &str) -> &Vec<Employee> {
            self.departments.get(department).unwrap()
        }
        pub fn add_employee_to_department(&mut self, employee: Employee, department: &str) {
            self.departments.get_mut(department).unwrap().push(employee);
        }
        pub fn delete_employee_from_department(&mut self, employee: Employee, department: &str) {
            let employees = self.departments.get_mut(department).unwrap();
            let index = employees
                .iter()
                .position(|e| e.get_name() == employee.get_name())
                .unwrap();
            employees.remove(index);
        }
    }
}

use std::process::Command;

use crate::company::Company;
use crate::employee::{Employee, Job, JobType};

fn main() {
    let menus = menus::Menu::new(
        String::from("Welcome to the company management system"),
        vec![
            String::from("Add new employee"),
            String::from("Add new department"),
            String::from("Add employee to department"),
            String::from("Delete employee from department"),
            String::from("Show all employees"),
            String::from("Show all employees by department"),
            String::from("Exit"),
        ],
    );
    let mut company_name = String::new();
    println!("Please enter the company name: ");
    std::io::stdin()
        .read_line(&mut company_name)
        .expect("Failed to read line");
    let mut company = Company::new(company_name.trim().to_string());
    loop {
        menus.show();
        let option = menus.get_option();
        match option {
            // Add new employee
            1 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                let mut name = String::new();
                println!("Please enter the employee name: ");
                std::io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let mut age = String::new();
                println!("Please enter the employee age: ");
                std::io::stdin()
                    .read_line(&mut age)
                    .expect("Failed to read line");
                let age: u8 = age.trim().parse().expect("Please type a number!");
                let mut title = String::new();
                println!("Please enter the employee job title: ");
                std::io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                let mut salary = String::new();
                println!("Please enter the employee salary: ");
                std::io::stdin()
                    .read_line(&mut salary)
                    .expect("Failed to read line");
                let salary: u32 = salary.trim().parse().expect("Please type a number!");
                println!("Please enter the employee job type (1 - Full time, 2 - Part time): ");
                let mut status = String::new();
                std::io::stdin()
                    .read_line(&mut status)
                    .expect("Failed to read line");
                let status = match status.trim().parse().expect("Please type a number!") {
                    1 => JobType::FullTime,
                    2 => JobType::PartTime,
                    _ => JobType::FullTime,
                };
                let job = Job {
                    title: title.trim().to_string(),
                    salary,
                    job_type: status,
                };
                let employee = Employee::new(name.trim().to_string(), age, job);
                company.add_employee(employee);
                menus.wait();
            }
            // Add new department
            2 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                let mut department = String::new();
                println!("Please enter the department name: ");
                std::io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                company.add_new_department(department.trim().to_string());
                menus.wait();
            }
            // Add employee to department
            3 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                let mut name = String::new();
                println!("Please enter the employee name: ");
                std::io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let mut department = String::new();
                println!("Please enter the department name: ");
                std::io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                let employee = company
                    .get_employees()
                    .iter()
                    .find(|e| e.get_name() == name.trim())
                    .unwrap();
                company.add_employee_to_department(employee.clone(), department.trim());
                menus.wait();
            }
            // Delete employee from department
            4 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                let mut name = String::new();
                println!("Please enter the employee name: ");
                std::io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let mut department = String::new();
                println!("Please enter the department name: ");
                std::io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                match company
                    .get_employees()
                    .iter()
                    .find(|e| e.get_name() == name.trim())
                {
                    Some(employee) => {
                        company.delete_employee_from_department(employee.clone(), department.trim())
                    }
                    None => println!("Employee not found"),
                }

                menus.wait();
            }
            // Show all employees
            5 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                for employee in company.get_employees() {
                    println!(
                        "Name: {}, Age: {}, Job: {}",
                        employee.get_name(),
                        employee.get_age(),
                        employee.get_job().title
                    );
                }
                menus.wait();
            }
            // Show all employees by department
            6 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                println!("======================================================================");
                let mut department = String::new();
                println!("Please enter the department name: ");
                std::io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                for employee in company.get_employees_by_department(department.trim()) {
                    println!(
                        "Name: {}, Age: {}, Job: {}",
                        employee.get_name(),
                        employee.get_age(),
                        employee.get_job().title
                    );
                }
                println!("======================================================================");

                menus.wait();
            }
            // Exit
            7 => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                println!("Goodbye!");
                menus.wait();
                break;
            }
            // Invalid option
            _ => {
                Command::new("clear")
                    .status()
                    .expect("Failed to clear screen");
                println!("Please enter a valid option!");
                menus.wait();
            }
        }
    }
}
