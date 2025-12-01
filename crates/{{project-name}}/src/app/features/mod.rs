/* To register a new feature, add a new module here
 * and update the register_features macro to include it.
 *
 * The macro will generate the necessary boilerplate code
 * for the feature's message enum, state struct, and window configuration.
 *
 * The feature's view and update functions will be automatically routed to the appropriate
 * functions in the app module.
 *
 * (Optional): If a new feature requires a separate window, add it to `ApplicationWindow`
 * enum in `windows` module. `register_features` macro will generate the necessary boilerplate
 * code for the window configuration.
 */
pub mod root;

use crate::register_features;

register_features! {
    Root => root { 768.0, 768.0, Centered },
}
