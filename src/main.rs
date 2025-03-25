use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use chrono::{Local, NaiveDate};

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    sex: String,
    height: f32,
    weight: f32,
}

#[derive(Debug, Clone)]
struct Product {
    unit: String,
    calories: f32,
    proteins: f32,
    minerals: f32,
}

#[derive(Debug, Clone)]
struct Meal {
    name: String,
    items: Vec<(String, f32)>,
    servings: f32,
}

#[derive(Debug, Clone)]
struct DailyLog {
    date: NaiveDate,
    meals: Vec<Meal>,
}

fn main() {
    let mut users: HashMap<String, User> = load_users();
    let mut products: HashMap<String, Product> = load_products();
    let mut meals: Vec<Meal> = load_meals();
    let mut daily_logs: HashMap<(String, NaiveDate), DailyLog> = load_daily_logs();

    println!("Welcome to Enhanced Diet Manager CLI");
    
    loop {
        println!("1. Sign Up\n2. Log In\n3. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => sign_up(&mut users),
            "2" => log_in(&users, &mut products, &mut meals, &mut daily_logs),
            "3" => {
                save_users(&users);
                save_products(&products);
                save_meals(&meals);
                save_daily_logs(&daily_logs);
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}


fn sign_up(users: &mut HashMap<String, User>) {
    println!("Enter Name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter Age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();
    let age: u32 = age.trim().parse().unwrap_or(0);

    println!("Enter Sex (M/F):");
    let mut sex = String::new();
    io::stdin().read_line(&mut sex).unwrap();

    println!("Enter Height (cm):");
    let mut height = String::new();
    io::stdin().read_line(&mut height).unwrap();
    let height: f32 = height.trim().parse().unwrap_or(0.0);

    println!("Enter Weight (kg):");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).unwrap();
    let weight: f32 = weight.trim().parse().unwrap_or(0.0);

    let user = User {
        name: name.trim().to_string(),
        age,
        sex: sex.trim().to_string(),
        height,
        weight,
    };
    users.insert(name.trim().to_string(), user);
    save_users(users);
    println!("User registered successfully!");
}

impl DailyLog {
    fn total_calories(&self, products: &HashMap<String, Product>) -> f32 {
        self.meals.iter()
            .flat_map(|meal| meal.items.iter().map(move |(product_name, quantity)| {
                products.get(product_name)
                    .map(|p| p.calories * quantity * meal.servings)
                    .unwrap_or(0.0)
            }))
            .sum()
    }

    fn total_nutrients(&self, products: &HashMap<String, Product>) -> (f32, f32) {
        let (total_proteins, total_minerals) = self.meals.iter()
            .flat_map(|meal| meal.items.iter().map(move |(product_name, quantity)| {
                products.get(product_name)
                    .map(|p| (p.proteins * quantity * meal.servings, p.minerals * quantity * meal.servings))
                    .unwrap_or((0.0, 0.0))
            }))
            .fold((0.0, 0.0), |(acc_p, acc_m), (p, m)| (acc_p + p, acc_m + m));
        
        (total_proteins, total_minerals)
    }
}


fn log_in(users: &HashMap<String, User>, products: &mut HashMap<String, Product>, meals: &mut Vec<Meal>, daily_logs: &mut HashMap<(String, NaiveDate), DailyLog>) {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    if users.contains_key(&name) {
        println!("Welcome back, {}!", name);
        user_menu(&name, products, meals, daily_logs);
    } else {
        println!("User not found.");
    }
}

fn user_menu(username: &str, products: &mut HashMap<String, Product>, meals: &mut Vec<Meal>, daily_logs: &mut HashMap<(String, NaiveDate), DailyLog>) {
    loop {
        println!("\nUser Menu:");
        println!("1. Manage Products");
        println!("2. Manage Meals");
        println!("3. Log Daily Intake");
        println!("4. View Daily Logs");
        println!("5. Go Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => manage_products(products),
            "2" => manage_meals(meals, products),
            "3" => log_daily_intake(username, products, meals, daily_logs),
            "4" => view_daily_logs(username, daily_logs, products),
            "5" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn manage_products(products: &mut HashMap<String, Product>) {
    loop {
        println!("\nProduct Management:");
        println!("1. Add Product");
        println!("2. View Products");
        println!("3. Go Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_product(products),
            "2" => view_products(products),
            "3" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn view_products(products: &HashMap<String, Product>) {
    println!("\n--- Products List ---");
    for (name, product) in products {
        println!("{}: {} {}, {} cal, {} proteins, {} minerals", 
            name, 1.0, product.unit, product.calories, product.proteins, product.minerals);
    }
}

fn view_meals(meals: &Vec<Meal>, products: &HashMap<String, Product>) {
    println!("\n--- Meals List ---");
    for (i, meal) in meals.iter().enumerate() {
        println!("Meal {}: {} (Servings: {})", i+1, meal.name, meal.servings);
        for (product_name, quantity) in &meal.items {
            if let Some(product) = products.get(product_name) {
                println!("  - {}: {} {} -> {:.2} cal, {:.2}g proteins, {:.2}g minerals", 
                    product_name, 
                    quantity * meal.servings, 
                    product.unit, 
                    product.calories * quantity * meal.servings, 
                    product.proteins * quantity * meal.servings, 
                    product.minerals * quantity * meal.servings
                );
            }
        }
    }
}


fn manage_meals(meals: &mut Vec<Meal>, products: &HashMap<String, Product>) {
    loop {
        println!("\nMeal Management:");
        println!("1. Add Meal");
        println!("2. View Meals");
        println!("3. Go Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_meal(products, meals),
            "2" => view_meals(meals, products),
            "3" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn add_product(products: &mut HashMap<String, Product>) {
    println!("Enter product name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!("Enter unit (slice/cup/g/100g/tablespoon/teaspoon):");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).unwrap();
    let unit = unit.trim().to_string();

    println!("Enter calories:");
    let calories: f32 = get_float_input("Invalid calories, using 0");

    println!("Enter proteins:");
    let proteins: f32 = get_float_input("Invalid proteins, using 0");

    println!("Enter minerals:");
    let minerals: f32 = get_float_input("Invalid minerals, using 0");

    let product = Product { unit, calories, proteins, minerals };
    products.insert(name.clone(), product);
    save_products(products);
    println!("Product '{}' added successfully!", name);
}

fn add_meal(products: &HashMap<String, Product>, meals: &mut Vec<Meal>) {
    println!("Enter meal name:");
    let mut meal_name = String::new();
    io::stdin().read_line(&mut meal_name).unwrap();
    let meal_name = meal_name.trim().to_string();

    let mut items = Vec::new();
    loop {
        println!("Enter product name (or 'done' to finish):");
        let mut product_name = String::new();
        io::stdin().read_line(&mut product_name).unwrap();
        let product_name = product_name.trim().to_string();
        for (key, product) in products.iter() {
            if key == &product_name {
            println!("Unit for input desirable is: {}", product.unit);
            }
        }
        if product_name == "done" { break; }

        if !products.contains_key(&product_name) {
            println!("Product not found. Please add the product first.");
            continue;
        }

        println!("Enter quantity:");
        let quantity: f32 = get_float_input("Invalid quantity, using 0");

        items.push((product_name, quantity));
    }

    println!("Enter number of servings for this meal:");
    let servings: f32 = get_float_input("Invalid servings, using 1");
    let servings = if servings > 0.0 { servings } else { 1.0 };

    let meal = Meal { 
        name: meal_name.clone(), 
        items, 
        servings 
    };
    let _total_calories: f32 = meal.items.iter()
    .map(|(product_name, quantity)| {
        products.get(product_name)
            .map(|p| p.calories * quantity * servings)
            .unwrap_or(0.0)
    })
    .sum();
    meals.push(meal);
    save_meals(meals);
    println!("Meal '{}' added successfully! Total Calories: {:.2}", meal_name, _total_calories);
    // println!("Meal '{}' added successfully!", meal_name);
}

fn log_daily_intake(username: &str, products: &HashMap<String, Product>, all_meals: &Vec<Meal>, daily_logs: &mut HashMap<(String, NaiveDate), DailyLog>) {
    let today = Local::now().date_naive();

    // Let user select from predefined meals or create a custom one
    println!("Select a meal to log:");
    println!("0. Create Custom Meal");
    for (i, meal) in all_meals.iter().enumerate() {
        println!("{}: {}", i+1, meal.name);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let meal = if choice == "0" {
        // Create custom meal on the fly
        create_custom_meal(products)
    } else if let Ok(index) = choice.parse::<usize>() {
        if index > 0 && index <= all_meals.len() {
            all_meals[index-1].clone()
        } else {
            println!("Invalid meal selection.");
            return;
        }
    } else {
        println!("Invalid choice.");
        return;
    };

    // Add the meal to daily log
    let log_key = (username.to_string(), today);
    daily_logs.entry(log_key)
        .or_insert(DailyLog { date: today, meals: Vec::new() })
        .meals.push(meal);
    save_daily_logs(daily_logs);
    println!("Meal logged successfully for today!");
}

fn create_custom_meal(products: &HashMap<String, Product>) -> Meal {
    println!("Creating a custom meal:");
    
    println!("Enter meal name:");
    let mut meal_name = String::new();
    io::stdin().read_line(&mut meal_name).unwrap();
    let meal_name = meal_name.trim().to_string();

    let mut items = Vec::new();
    loop {
        println!("Enter product name (or 'done' to finish):");
        let mut product_name = String::new();
        io::stdin().read_line(&mut product_name).unwrap();
        let product_name = product_name.trim().to_string();
        if product_name == "done" { break; }

        if !products.contains_key(&product_name) {
            println!("Product not found. Please add the product first.");
            continue;
        }

        println!("Enter quantity:");
        let quantity: f32 = get_float_input("Invalid quantity, using 0");

        items.push((product_name, quantity));
    }

    println!("Enter number of servings:");
    let servings: f32 = get_float_input("Invalid servings, using 1");
    let servings = if servings > 0.0 { servings } else { 1.0 };

    Meal { 
        name: meal_name, 
        items, 
        servings 
    }
}

fn load_meals() -> Vec<Meal> {
    let mut meals = Vec::new();
    if let Ok(data) = fs::read_to_string("meals.txt") {
        let mut lines = data.lines();
        while let Some(meal_name) = lines.next() {
            let mut meal_items = Vec::new();
            while let Some(line) = lines.next() {
                if line.is_empty() { break; }
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 {
                    meal_items.push((parts[0].to_string(), parts[1].parse().unwrap_or(0.0)));
                }
            }
            meals.push(Meal {
                name: meal_name.to_string(),
                items: meal_items,
                servings: 1.0, // Default servings
            });
        }
    }
    meals
}

fn view_daily_logs(username: &str, daily_logs: &HashMap<(String, NaiveDate), DailyLog>, products: &HashMap<String, Product>) {
    // Sort dates in descending order
    let mut user_logs: Vec<_> = daily_logs.iter()
        .filter(|((name, _), _)| name == username)
        .collect();
    
    user_logs.sort_by(|a, b| b.0.1.cmp(&a.0.1));

    if user_logs.is_empty() {
        println!("No daily logs found.");
        return;
    }

    for ((_, date), log) in user_logs {
        println!("\n--- Daily Log for {} ---", date);
        
        for (i, meal) in log.meals.iter().enumerate() {
            println!("Meal {}: {} (Servings: {})", i+1, meal.name, meal.servings);
            for (product_name, quantity) in &meal.items {
                if let Some(product) = products.get(product_name) {
                    println!("  - {}: {} {} -> {:.2} cal, {:.2}g proteins, {:.2}g minerals", 
                        product_name, 
                        quantity * meal.servings, 
                        product.unit, 
                        product.calories * quantity * meal.servings, 
                        product.proteins * quantity * meal.servings, 
                        product.minerals * quantity * meal.servings
                    );
                }
            }
        }

        let total_calories = log.total_calories(products);
        let (total_proteins, total_minerals) = log.total_nutrients(products);

        println!("\nTotal Daily Intake:");
        println!("Calories: {:.2}", total_calories);
        println!("Proteins: {:.2}g", total_proteins);
        println!("Minerals: {:.2}g", total_minerals);
    }
}

fn get_float_input(error_msg: &str) -> f32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("{}", error_msg);
                return 0.0;
            }
        }
    }
}

fn load_daily_logs() -> HashMap<(String, NaiveDate), DailyLog> {
    let mut daily_logs = HashMap::new();
    if let Ok(data) = fs::read_to_string("daily_logs.txt") {
        for line in data.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3 {
                let username = parts[0].to_string();
                let date = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d").unwrap();
                let servings = parts[2].parse::<f32>().unwrap_or(1.0);
                let product_name = parts[3].to_string();
                let quantity = parts[4].parse::<f32>().unwrap_or(0.0);

                let log_key = (username.clone(), date);
                daily_logs.entry(log_key)
                    .or_insert(DailyLog { date, meals: Vec::new() })
                    .meals.push(Meal {
                        name: "Loaded Meal".to_string(),
                        items: vec![(product_name, quantity)],
                        servings,
                    });
            }
        }
    }
    daily_logs
}

fn save_daily_logs(daily_logs: &HashMap<(String, NaiveDate), DailyLog>) {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("daily_logs.txt").unwrap();
    for ((username, date), log) in daily_logs {
        for meal in &log.meals {
            for (product_name, quantity) in &meal.items {
                writeln!(file, "{},{},{},{},{}", 
                    username, 
                    date.format("%Y-%m-%d"), 
                    meal.servings, 
                    product_name, 
                    quantity
                ).unwrap();
            }
        }
    }
}



fn load_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    if let Ok(data) = fs::read_to_string("users.txt") {
        for line in data.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 5 {
                let user = User {
                    name: parts[0].to_string(),
                    age: parts[1].parse().unwrap_or(0),
                    sex: parts[2].to_string(),
                    height: parts[3].parse().unwrap_or(0.0),
                    weight: parts[4].parse().unwrap_or(0.0),
                };
                users.insert(user.name.clone(), user);
            }
        }
    }
    users
}

fn save_users(users: &HashMap<String, User>) {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("users.txt").unwrap();
    for user in users.values() {
        writeln!(file, "{},{},{},{},{}", user.name, user.age, user.sex, user.height, user.weight).unwrap();
    }
}

fn save_meals(meals: &Vec<Meal>) {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("meals.txt").unwrap();
    for meal in meals {
        writeln!(file, "{}", meal.name).unwrap();
        for (product_name, quantity) in &meal.items {
            writeln!(file, "{},{}", product_name, quantity).unwrap();
        }
        writeln!(file, "").unwrap();
    }
}

fn load_products() -> HashMap<String, Product> {
    let mut products = HashMap::new();
    if let Ok(data) = fs::read_to_string("products.txt") {
        for line in data.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 5 {
                let key = parts[0].to_string();
                let product = Product {
                    unit: parts[1].to_string(),
                    calories: parts[2].parse().unwrap_or(0.0),
                    proteins: parts[3].parse().unwrap_or(0.0),
                    minerals: parts[4].parse().unwrap_or(0.0),
                };
                products.insert(key, product);
            }
        }
    }
    products
}

fn save_products(products: &HashMap<String, Product>) {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("products.txt").unwrap();
    for (key, product) in products.iter() {
        writeln!(
            file,
            "{},{},{},{},{}",
            key, product.unit, product.calories, product.proteins, product.minerals
        )
        .unwrap();
    }
}
