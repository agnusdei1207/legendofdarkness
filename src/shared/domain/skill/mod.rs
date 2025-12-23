pub mod models;
pub mod server; // Must be available to both CSR/SSR for the stub


// Re-exporting the struct specific to the server function if needed, 
// but usually we import the function 'get_skills' (or the struct GetSkills which the macro generates).
pub use server::get_skills;
