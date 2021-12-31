fn main() {
    let mut list_of_employees = ListOfEmployees::new();
    list_of_employees=list_of_employees.fill_in();
    for e in &list_of_employees.list {
    println!("{}:{}", e.name,e.department);
    }
    let department_list=&list_of_employees.department_list();
    let company_list=list_of_employees.company_list();
}
use std::io;
//use std::collections::HashMap;
pub fn input() -> String {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            return input.trim().to_string();
}

struct Employee {
    name: String,
    department:String,
}
impl Employee {
    fn new() -> Employee {
        println!("Input name (last name, first name)");
        let name = input();
        println!("Input a department");
        let department=input();
        Employee {
            name: name,
            department: department,
        }
    }
}
struct ListOfEmployees {
    list: Vec<Employee>
}
impl ListOfEmployees {
    fn new()->ListOfEmployees {
        let list: Vec<Employee> = Vec::new();
        ListOfEmployees {
                list: list,
        }
    }
    fn fill_in(self) ->Self {
        let mut list = Self::new();
        loop {
            let employee = Employee::new();
            list=list.add_employee(employee);
            println!("Add new employee: y/n");
            let add=input();
            if add == "n".to_string() {
                break;
            }
        }
        list
    }
    fn add_employee(self, employee: Employee) -> Self {
        let mut l = self.list;
        l.push(employee);
        Self {
            list:l,
        }      
    }
    fn list_of_departments(&self) ->Vec<String> {
        let mut v: Vec<String>=Vec::new();
        for e in &self.list {
            let mut find = false;
            for i in &v {
                if *e.department==*i {
                    find=true;
                    break;
                }
            }
            if find == false {
                v.push((e.department).to_string());
            }
        }
        v
    }
    fn department_list(&self) ->Vec<String>{
        println!("Choose a department:");
        for i in &self.list_of_departments() {
            println!("{}",i);
        };
        let department=input();
        //let l=&self.list;
        //let list: Vec<Employee> = *l.into_iter().filter(|s| s.department == department).collect();
        let mut department_list:Vec<String> =Vec::new();
        for i in &self.list {
            if *i.department  == department {
                 department_list.push((&i.name).to_string());
            }
        }
        department_list.sort();
        for i in &department_list {
            println!("{}",i);
        };
        department_list
    }
    fn company_list(self)-> Vec<Employee> {
        let list = (self.list).sort();
        for i in &list {
            println!("{}:{}", i.name, i.department);
        }
        list
    }
}


//#[cfg(tests)]
//mod tests {
    //use super::*;
    //#[test]
    //fn department_list_test {
        //let e1 = Employee {"Smith Mary". to_string(),"Sale".to_string,};
        //let e2 = Employee {"Smith John".to_string(), "Sale".to_string(),};
        //let e3 = Employee {"Dow John".to_string(), "IT".to_string(),};
        //let mut list = vec![e1,e2,e3];
        //list.department_list();
    //}

