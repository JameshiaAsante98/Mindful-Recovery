#![allow(non_snake_case)]
use std::collections::HashMap;

// Mindful Recovery
// Mindful recovery is a non-medical approach to treating addiction that focuses on developing a deep understanding of the causes of addiction and how to take control of one's own recovery.

fn main () {
    // Create a HashMap to store information about the causes of addiction
    let mut addiction_causes = HashMap::new();
    addiction_causes.insert("genetic", "Genetic predisposition");
    addiction_causes.insert("environmental", "Environmental factors");
    addiction_causes.insert("psychological", "Psychological and emotional issues");
    addiction_causes.insert("social", "Social influences");
    addiction_causes.insert("biological", "Biological imbalances");
    
    // Create a vector to store the steps of mindful recovery
    let mut mindfulRecoverySteps: Vec<String> = Vec::new();
    mindfulRecoverySteps.push("Acceptance".to_string());
    mindfulRecoverySteps.push("Mindfulness".to_string());
    mindfulRecoverySteps.push("Awareness".to_string());
    mindfulRecoverySteps.push("Self-Exploration".to_string());
    mindfulRecoverySteps.push("Reflection".to_string());
    mindfulRecoverySteps.push("Commitment".to_string());
    
    // Create a variable to store the number of mindful recovery steps
    let numSteps = mindfulRecoverySteps.len();
    
    // Create a function to print out the steps of mindful recovery
    fn print_mindful_recovery_steps(steps: &Vec<String>, num_steps: usize) {
        println!("The steps of mindful recovery are:");
        for i in 0..num_steps {
            println!("{}. {}", i + 1, steps[i]);
        }
    }
    
    // Create a function to list the causes of addiction 
    fn list_addiction_causes(causes: &HashMap<&str, &str>) {
        println!("The causes of addiction are:");
        for (key, value) in causes.iter() {
            println!("{}: {}", key, value);
        }
    }
    
    // Print out the steps of mindful recovery and list the causes of addiction
    print_mindful_recovery_steps(&mindfulRecoverySteps, numSteps);
    list_addiction_causes(&addiction_causes);
    
    // Create a function to demonstrate the power of mindful recovery
    fn demonstrate_mindful_recovery(causes: &HashMap<&str, &str>, steps: &Vec<String>) {
        println!("\nMindful recovery has the power to help an individual gain insight into their addiction and how to overcome it. By understanding the causes of addiction and practicing the steps of mindful recovery, an individual can take control of their own recovery and begin to heal.");
        for (key, value) in causes.iter() {
            println!("\nA key step in mindful recovery is to understand the cause of addiction, such as {}: {}", key, value);
        }
        println!("\nOnce the cause is understood, the individual can begin to practice the steps of mindful recovery:");
        for step in steps {
            println!("- {}", step);
        }
    }
    
    // Demonstrate the power of mindful recovery 
    demonstrate_mindful_recovery(&addiction_causes, &mindfulRecoverySteps);
}