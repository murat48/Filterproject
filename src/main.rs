use rand::Rng;

fn gettype(number1: u8, number2: u8) -> ComparisonType {
    if number1 > number2 {
        return ComparisonType::GreaterThan;
    } else if number1 < number2 {
        return ComparisonType::LessThan;
    } else {
        return ComparisonType::EqualTo;
    }
}

// Define a generic FilterCondition struct
struct FilterCondition<T> {
    threshold: T,
    comparison_type: ComparisonType,
}

// Enum to define different comparison types
#[derive(Debug)]
enum ComparisonType {
    GreaterThan,
    LessThan,
    EqualTo,
}

// Implement is_match method for the FilterCondition struct
impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self, value: &T) -> bool {
        match self.comparison_type {
            ComparisonType::GreaterThan => value > &self.threshold,
            ComparisonType::LessThan => value < &self.threshold,
            ComparisonType::EqualTo => value == &self.threshold,
        }
    }
}

// Generic custom_filter function
fn custom_filter<T: PartialOrd>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: Clone,
{
    collection
        .iter()
        .filter(|&x| condition.is_match(x))
        .cloned()
        .collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut rng = rand::thread_rng();

    let random_number1 = rng.gen_range(1..=10);
    let random_number2 = rng.gen_range(1..=10);
    let resultnum: String;
    if random_number1 > random_number2 {
        resultnum = format!(
            "{} Numbers greater than {} ",
            random_number1, random_number2
        );
    } else if random_number1 < random_number2 {
        resultnum = format!(
            "{} Numbers LessThan than {}",
            random_number1, random_number2
        );
    } else {
        resultnum = format!("{} Numbers EqualTo {}", random_number1, random_number2);
    }
    let condition = FilterCondition {
        threshold: random_number1,
        comparison_type: gettype(random_number1, random_number2),
    };

    let filtered_numbers = custom_filter(&numbers, &condition);

    println!(" {resultnum} : {:?}", filtered_numbers);
}
