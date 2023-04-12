use zero_pass_backend::{ Methods, encrypt };
use yew::prelude::*;

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

    let pass = "something";

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
                            <input type="text" class="form-control form-control-lg mb-3 text-bg-dark border-primary-subtle" id="UniquePassword" placeholder="unique" />
                            <label for="VariablePassword" >{"Variable pass"}</label>
                            <input type="text" class="form-control form-control-lg mb-3 text-bg-dark border-primary-subtle" id="VariablePassword" placeholder="variable" />
                            <label for="EncryptionMethod" >{"Method"}</label>
                            <div class="row mb-3">
                                <div class="col">
                                    <select class="form-select form-select-lg text-bg-dark border-primary-subtle" id="EncryptionMethod" >
                                        {method_options()}
                                    </select>
                                </div>
                                <div class="col-3">
                                    <input class="form-control text-bg-dark border-secondary-subtle" type="number" value="0" />
                                </div>
                            </div>
                            <p class="text-center"><b>{pass}</b></p>
                            <button type="button" class="btn btn-primary w-100">{"Generate password"}</button>
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
