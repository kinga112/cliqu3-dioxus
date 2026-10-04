use dioxus::prelude::*;
use iroh_live::media::AudioBackend;

#[component]
pub fn Audio() -> Element {
    let inputs = AudioBackend::list_inputs();
    let outputs = AudioBackend::list_outputs();

    let input_devices = inputs.iter().map(|device| {
        rsx! {
            option {
                "{device.name}"
            }
        }
    });

    let output_devices = outputs.iter().map(|device| {
        rsx! {
            option {
                "{device.name}"
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col bg-off-black-500 w-full p-20 gap-10",
            div{
                class: "flex flex-col gap-2 text-xl",
                "Input Devices"
                div{
                    class: "flex flex-col text-sm",
                    "Current Device: {inputs[0].name}"
                    select {
                        class: "w-52 border-2 border-deep-purple-300 rounded truncate",
                        {input_devices}
                    }
                }
            }
            div{
                class: "flex flex-col gap-2 text-xl",
                "Output Devices"
                div{
                    class: "flex flex-col text-sm",
                    "Current Device: {outputs[0].name}"
                    select {
                        class: "w-52 border-2 border-deep-purple-300 rounded truncate",
                        {output_devices}
                    }
                }
            }
        }
    }
}
