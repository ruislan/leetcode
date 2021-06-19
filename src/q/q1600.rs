use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: String,
    alive: bool,
    children: Vec<Option<Rc<RefCell<Person>>>>,
}

struct ThroneInheritance {
    root: Option<Rc<RefCell<Person>>>,
    cache: HashMap<String, Option<Rc<RefCell<Person>>>>,
}

/**
 * 方法1
 * 将所有的家族用树表示出来，
 * 根据题意可以看出来，如果没有死亡发生的话，其实是一个前序遍历
 * 然后我们把死亡的标记出来，如果有死亡，就不放进继承列表里面
 * AC 208ms 68.5mb
 */
#[allow(unused)]
impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        let root = Some(Rc::new(RefCell::new(
            Person {
                name: king_name.clone(),
                alive: true,
                children: Vec::new(),
            })));
        let mut cache = HashMap::new();
        cache.insert(king_name, root.clone());
        ThroneInheritance {
            root,
            cache,
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        let new_person = Some(Rc::new(RefCell::new(Person {
            name: child_name.clone(),
            alive: true,
            children: Vec::new(),
        })));
        self.cache.insert(child_name, new_person.clone());
        if let Some(person) = self.cache.get_mut(&parent_name) {
            person.as_mut().unwrap().borrow_mut().children.push(new_person);
        }
    }

    fn death(&mut self, name: String) {
        if let Some(person) = self.cache.get_mut(&name) {
            person.as_mut().unwrap().borrow_mut().alive = false;
        }
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut stack = vec![self.root.clone()];
        let mut orders = Vec::new();
        while !stack.is_empty() {
            let node = stack.pop();
            let node = node.unwrap().clone().unwrap();
            if node.borrow().alive { orders.push(node.borrow().name.clone()); }
            for child in node.borrow().children.iter().rev() {
                stack.push(child.clone());
            }
        }
        orders
    }
}