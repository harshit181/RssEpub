use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_message = use_signal(|| Option::<String>::None);
    let navigator = use_navigator();

    let handle_submit = move |_| {
        to_owned![username, password, error_message, navigator];
        async move {
            // Call the server function to validate the login
            match validate_login(username.read().clone(), password.read().clone()).await {
                Ok(true) => {
                    // Successful login
                    error_message.set(None);
                    navigator.push("/"); // Navigate to the home route
                }
                Ok(false) => {
                    // Invalid credentials
                    error_message.set(Some("Invalid username or password.".to_string()));
                }
                Err(e) => {
                    // Server error
                    error_message.set(Some(format!("Login error: {}", e)));
                }
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}
        div {
            class: "flex justify-center items-center min-h-screen bg-gray-100", // Tailwind classes for layout
            div {
                class: "bg-white p-8 rounded-lg shadow-md w-96", // Tailwind classes for form container
                form {
                    onsubmit: handle_submit,
                    class: "space-y-4", // Tailwind class for spacing form elements
                    h2 {
                        class: "text-2xl font-bold text-center text-gray-800", // Tailwind classes for heading
                        "Login"
                    }
                    div {
                        class: "space-y-1", // Tailwind class for spacing inside input group
                        label {
                            class: "block text-sm font-medium text-gray-700", // Tailwind classes for label
                            r#for: "username",
                            "Username"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500", // Tailwind classes for input
                            r#type: "text",
                            id: "username",
                            name: "username",
                            value: "{username}",
                            oninput: move |event| username.set(event.value()),
                            required: true,
                        }
                    }
                    div {
                        class: "space-y-1", // Tailwind class for spacing inside input group
                        label {
                            class: "block text-sm font-medium text-gray-700", // Tailwind classes for label
                            r#for: "password",
                            "Password"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500", // Tailwind classes for input
                            r#type: "password",
                            id: "password",
                            name: "password",
                            value: "{password}",
                            oninput: move |event| password.set(event.value()),
                            required: true,
                        }
                    }
                    if let Some(err) = error_message() {
                        div {
                            class: "text-red-500", // Tailwind class for error message text
                            "{err}"
                        }
                    }
                    button {
                        class: "w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:shadow-outline", // Tailwind classes for button
                        r#type: "submit",
                        "Login"
                    }
                }
            }
        }
    }
}

/// Validates the user login on the server.
#[server(ValidateLogin)]
async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    // Replace this with your actual authentication logic
    // Example: Check against a database or an authentication service
    if username == "test" && password == "password" {
        Ok(true)
    } else {
        Ok(false)
    }
}