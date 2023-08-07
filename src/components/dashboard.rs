use leptos::*;

#[slot]
pub struct MainSlot {
    children: ChildrenFn,
}

#[component]
pub fn DashBoard(cx: Scope, main_slot: MainSlot) -> impl IntoView {
    view! { cx,
        <h1>"Dashboard"</h1>
        {move || (main_slot.children)(cx).into_view(cx)}
        <Crud />
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    surname: String,
    id: usize,
}

#[component]
pub fn Crud(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (surname, set_surname) = create_signal(cx, String::new());

    let (persons, set_persons) = create_signal(cx, vec![]);

    let mut id = 0;

    view! { cx,
        <div>
            <h1>Crud</h1>
            <div>
                <label>Name</label>
                <input type="text" on:input=move |ev| set_name(event_target_value(&ev)) prop:value=name />
            </div>

            <div>
                <label>Surname</label>
                <input type="text" on:input=move |ev| set_surname(event_target_value(&ev)) prop:value=surname />
            </div>

            <button on:click=move |_| {
                set_persons.update(|ps| ps.push(Person {
                    name: name(),
                    surname: surname(),
                    id: id + 1,
                }));
                set_name(String::new());
                set_surname(String::new());
                id += 1;
            }>"Add Person"</button>

            <ul>
                <For
                    each=persons
                    key=|person| person.id
                    view=move |cx, person| {
                        view! { cx,
                            <li>{person.name} {person.surname}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
