use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;

type SlotInfo = HashMap<String, Vec<(String, String)>>;
type Registered = HashMap<String, SlotInfo>;

lazy_static! {
    static ref REGISTERED: RwLock<Registered> = RwLock::new(HashMap::new());
}

/// This struct manages the hooks. It basically provides two functions: adding
/// slots and emitting signals.
pub struct Hook;

impl Hook {
    /// Connects a function to a hook
    ///
    /// This function makes it very easy to connect to use hooks.
    ///
    /// # Arguments
    /// * `signal_class` - class name of emitter
    /// * `signal_name` - name of signal
    /// * `slot_class` - class name of slot
    /// * `slot_name` - name of slot
    ///
    /// # Returns
    /// Always returns true as there's no chance for failure
    ///
    /// TODO: write example
    pub fn connect(signal_class: &str, signal_name: &str, slot_class: &str, slot_name: &str) -> bool {
        let mut registered = REGISTERED.write().unwrap();
        
        // If we're trying to connect to an emitting class that isn't
        // yet registered, register it
        let class_signals = registered.entry(signal_class.to_string()).or_insert_with(HashMap::new);
        
        // If we're trying to connect to an emitting method that isn't
        // yet registered, register it with the emitting class
        let signal_slots = class_signals.entry(signal_name.to_string()).or_insert_with(Vec::new);
        
        // Connect the hook handler to the requested emitter
        signal_slots.push((slot_class.to_string(), slot_name.to_string()));
        
        // No chance for failure ;-)
        true
    }

    /// Emits a signal
    ///
    /// Emits a signal. To get data from the slot use references!
    ///
    /// # Arguments
    /// * `signal_class` - class name of emitter
    /// * `signal_name` - name of signal
    /// * `params` - HashMap with additional data
    ///
    /// # Returns
    /// Returns true if slots exist or false if not
    ///
    /// TODO: write example
    pub fn emit(signal_class: &str, signal_name: &str, params: &mut HashMap<String, Box<dyn std::any::Any>>) -> bool {
        let registered = REGISTERED.read().unwrap();
        
        // Return false if no hook handlers are listening to this emitting class
        let class_signals = match registered.get(signal_class) {
            Some(signals) => signals,
            None => return false,
        };
        
        // Return false if no hook handlers are listening to this emitting method
        let slots = match class_signals.get(signal_name) {
            Some(slots) => slots,
            None => return false,
        };
        
        // Call all slots
        for (slot_class, slot_name) in slots {
            match call_hook_function(slot_class, slot_name, params) {
                Ok(_) => {},
                Err(e) => log::error!(
                    "Error while running hook ({}::{}): {}", 
                    slot_class, 
                    slot_name, 
                    e
                ),
            }
        }
        
        // return true
        true
    }

    /// Clear hooks
    ///
    /// # Arguments
    /// * `signal_class` - Optional class name to clear
    /// * `signal_name` - Optional signal name to clear
    pub fn clear(signal_class: Option<&str>, signal_name: Option<&str>) {
        let mut registered = REGISTERED.write().unwrap();
        
        match (signal_class, signal_name) {
            (Some(class), Some(name)) => {
                if let Some(class_signals) = registered.get_mut(class) {
                    if class_signals.contains_key(name) {
                        class_signals.insert(name.to_string(), Vec::new());
                    }
                }
            },
            (Some(class), None) => {
                if registered.contains_key(class) {
                    registered.insert(class.to_string(), HashMap::new());
                }
            },
            (None, _) => {
                registered.clear();
            }
        }
    }
}

// This function would need to be implemented based on how you want to call hook functions
fn call_hook_function(
    class_name: &str, 
    method_name: &str, 
    params: &mut HashMap<String, Box<dyn std::any::Any>>
) -> Result<(), String> {
    // Implementation would depend on your application architecture
    // This is a placeholder that would need to be replaced with actual implementation
    Ok(())
}