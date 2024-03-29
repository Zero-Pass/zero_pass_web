use zero_pass_backend::{ Methods, encrypt };
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};

fn method_options() -> Vec<Html> {
    let method_vec = Methods::get_methods();
    let mut option_vec: Vec<Html> = Vec::new();

    for method in method_vec {
        option_vec.push(html!{<option value={method} >{method}</option>});
    }
    option_vec
}

#[function_component(App)]
fn app() -> Html {

    let password_result = use_state(String::default);
    let password_result_value = (*password_result).clone();

    let unique_value_state = use_state(|| String::from("u"));
    let unique_value = (*unique_value_state).clone();
    let unique_input_handle = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Event should have a target when dispatched");
        let unique_value_state = unique_value_state.clone();
        unique_value_state.set(target.unchecked_into::<HtmlInputElement>().value());
    }); 

    let variable_value_state = use_state(|| String::from("v"));
    let variable_value = (*variable_value_state).clone();
    let variable_input_handle = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Event should have a target when dispatched");
        let variable_value_state = variable_value_state.clone();
        variable_value_state.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    let method_value_state = use_state(|| String::from("Base64"));
    let method_value = (*method_value_state).clone();
    let method_select_handle = Callback::from(move |e: Event| {
        let target = e.target().expect("Event should have a target when dispatched");
        let method_value_state = method_value_state.clone();
        method_value_state.set(target.unchecked_into::<HtmlSelectElement>().value());
    });

    let repeat_value_state = use_state(|| 0u8);
    let repeat_value = (*repeat_value_state).clone();
    let repeat_increment_handle = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Event should have a target when dispatched");
        let repeat_value_state = repeat_value_state.clone();
        let value = target.unchecked_into::<HtmlInputElement>().value().parse::<u8>().unwrap();
        repeat_value_state.set(value);
    });

    let onclick = move |_| {
        let method = Methods::get_method(method_value.clone()).unwrap();
        let pass = encrypt::PasswordBuilder::new()
            .unique(unique_value.clone())
            .variable(variable_value.clone())
            .method_ptr(method).expect("Error")
            .repeat(repeat_value)
            .build();
        password_result.set(pass);
    };

    html!{
        <>
            <script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/js/bootstrap.bundle.min.js"
                integrity="sha384-ENjdO4Dr2bkBIFxQpeoTz1HIcje39Wm4jDKdf19U8gI4ddQ3GYNS7NTKfAdVQSZe"
                crossorigin="anonymous"></script>

            <main>
                <div class="position-absolute top-50 start-50 translate-middle w-100">
                    <h2 class="m-3 text-danger-emphasis">{"Zero Pass"}</h2>
                    <div class="card text-bg-dark p-3 rounded-4 shadow">
                        <div class="card-body p-2">
                            <label class="form-label" for="UniquePassword" >{"Unique pass"}</label>
                            <input onchange={unique_input_handle} type="text" class="form-control form-control-lg mb-3 text-bg-dark border-primary-subtle" id="UniquePassword" placeholder="unique" />
                            <label for="VariablePassword" >{"Variable pass"}</label>
                            <input onchange={variable_input_handle} type="text" class="form-control form-control-lg mb-3 text-bg-dark border-primary-subtle" id="VariablePassword" placeholder="variable" />
                            <label for="EncryptionMethod" >{"Method"}</label>
                            <div class="row mb-3">
                                <div class="col">
                                    <select onchange={method_select_handle} class="form-select form-select-lg text-bg-dark border-primary-subtle" id="EncryptionMethod" >
                                        {method_options()}
                                    </select>
                                </div>
                                <div class="col-3">
                                    <input onchange={repeat_increment_handle} class="form-control text-bg-dark border-secondary-subtle" type="number" value="0" />
                                </div>
                            </div>
                            <p class="text-center"><b>{ password_result_value }</b></p>
                            <button {onclick} type="button" class="btn btn-primary w-100">{"Generate password"}</button>
                        </div>
                    </div>
                </div>
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
