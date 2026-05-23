// src/ui/calculator_decoy.rs
//! Phase 8 — Social Stealth (Calculator Decoy UI)
//!
//! A functional calculator UI that serves as a disguise for the EdgeRay app.
//! Entering a specific secret sequence (e.g., "5555=") unlocks the real
//! proxy configuration interface.

pub struct CalculatorDecoy {
    display_value: String,
    secret_trigger: String,
    current_input: String,
    is_unlocked: bool,
}

impl CalculatorDecoy {
    pub fn new() -> Self {
        Self {
            display_value: "0".to_string(),
            secret_trigger: "5555=".to_string(),
            current_input: String::new(),
            is_unlocked: false,
        }
    }

    /// Process a button press. Returns true if the real UI should be unlocked.
    pub fn press_button(&mut self, btn: &str) -> bool {
        if self.is_unlocked {
            return true;
        }

        self.current_input.push_str(btn);

        // Keep input buffer bounded
        if self.current_input.len() > 20 {
            self.current_input = self.current_input[self.current_input.len() - 20..].to_string();
        }

        if self.current_input.ends_with(&self.secret_trigger) {
            self.is_unlocked = true;
            return true;
        }

        // Basic calculator display logic (stubbed for brevity)
        if btn == "C" {
            self.display_value = "0".to_string();
            self.current_input.clear();
        } else {
            self.display_value = btn.to_string();
        }

        false
    }

    pub fn is_unlocked(&self) -> bool {
        self.is_unlocked
    }

    pub fn get_display(&self) -> &str {
        &self.display_value
    }
}
